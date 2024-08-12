use std::{sync::Arc, time::Duration};

use async_std::sync::Mutex;
use async_trait::*;
use enclave_core::{
    create_event_system, EnclaveEvent, EnclaveEventType, EventConsumer, EventProducer,
    EventRuntime, OutputDecrypted,
};

// Some loose error/result stuff we can use
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

struct TestHandler {
    received: Arc<Mutex<Vec<EnclaveEvent>>>,
}

#[async_trait]
impl EventConsumer for TestHandler {
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
    let handler = TestHandler {
        received: received.clone(),
    };

    subscriber.subscribe(EnclaveEventType::OutputDecrypted, Box::new(handler));

    let event_loop = tokio::spawn(async move {
        subscriber.run().await.unwrap();
    });

    publisher
        .emit(EnclaveEvent::OutputDecrypted(OutputDecrypted {
            output: "hello world".to_string(),
        }))
        .await?;

    // Wait a bit for events to be processed
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Stop the event loop
    event_loop.abort();

    let received = received.lock().await;

    assert_eq!(received.len(), 1);
    assert!(matches!(received[0], EnclaveEvent::OutputDecrypted(_)));

    Ok(())
}

// test_ciphernode

#[tokio::test]
async fn test_key_aggregation() -> Result<()> {
    let (publisher, mut subscriber) = create_event_system();

    Ok(())
}
