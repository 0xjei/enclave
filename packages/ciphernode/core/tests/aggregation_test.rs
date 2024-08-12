use std::{sync::Arc, time::Duration};

use async_std::sync::Mutex;
use async_trait::*;
use enclave_core::{
    create_event_system, Ciphernode, ComputationRequested, EnclaveEvent, EnclaveEventType,
    EventConsumer, EventProducer, EventRuntime, OutputDecrypted,
};

// Some loose error/result stuff we can use
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct EventReporter {
    received: Arc<Mutex<Vec<EnclaveEvent>>>,
}

#[async_trait]
impl EventConsumer for EventReporter {
    async fn consume(&self, event: EnclaveEvent) -> Result<()> {
        self.received.lock().await.push(event);
        Ok(())
    }
}

// This tests an event system we will use to bind our test system together to avoid the need for
// running integration infrastructure such as libp2p in order to test our core logic
#[tokio::test]
async fn test_event_publisher() -> Result<()> {
    let (publisher, mut subscriber) = create_event_system();
    let received = Arc::new(Mutex::new(Vec::new()));
    let reporter = EventReporter {
        received: received.clone(),
    };

    subscriber.subscribe(EnclaveEventType::OutputDecrypted, Box::new(reporter));

    let event_loop = tokio::spawn(async move {
        subscriber.run().await.unwrap();
    });

    publisher
        .emit(EnclaveEvent::OutputDecrypted(OutputDecrypted {
            output: "hello world".to_string(),
        }))
        .await?;

    // Wait a tick for events to be processed
    tokio::time::sleep(Duration::from_millis(0)).await;

    // Stop the event loop
    event_loop.abort();

    let received = received.lock().await;

    assert_eq!(received.len(), 1);
    assert!(matches!(received[0], EnclaveEvent::OutputDecrypted(_)));

    Ok(())
}

#[tokio::test]
async fn test_key_aggregation() -> Result<()> {
    // NOTE: Under construction
    
    let (publisher, mut subscriber) = create_event_system();
    let received = Arc::new(Mutex::new(Vec::new()));
    let reporter = EventReporter {
        received: received.clone(),
    };
    let ciphernode = Ciphernode::new(publisher.clone())?;

    // Setup ciphernode system.
    subscriber.subscribe(EnclaveEventType::ComputationRequested, Box::new(ciphernode));

    // Listen to events with the reporter
    subscriber.subscribe(
        EnclaveEventType::ComputationRequested,
        Box::new(reporter.clone()),
    );
    subscriber.subscribe(EnclaveEventType::KeyshareCreated, Box::new(reporter.clone()));


    // Setup the loop
    let event_loop = tokio::spawn(async move {
        subscriber.run().await.unwrap();
    });

    // Publish the first event
    publisher
        .emit(EnclaveEvent::ComputationRequested(ComputationRequested {
            e3_id: "1234".to_string(),
            ciphernode_group_length: 100,
            ciphernode_threshold: 50,
            sortition_seed: 1234567,
        }))
        .await?;

    // Wait a bit for events to be processed
    tokio::time::sleep(Duration::from_millis(0)).await;

    // Stop the event loop
    event_loop.abort();

    let received = received.lock().await;

    assert_eq!(received.len(), 2);

    // First event dispatches
    assert!(matches!(received[0], EnclaveEvent::ComputationRequested(_)));
    // Ciphernode dispatches keyshare created
    assert!(matches!(received[1], EnclaveEvent::KeyshareCreated(_)));
    Ok(())
}
