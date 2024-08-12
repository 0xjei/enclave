use crate::fhe::PublicKeyShare;
use async_trait::*;
use ethereum_types::Address;
use mockall::automock;
use std::collections::{HashMap, VecDeque};
use tokio::sync::mpsc::{self, unbounded_channel};
// use tokio::time::{sleep, Duration};

// Some loose error/result stuff we can use
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct KeyshareCreated {
    pub pubkey: PublicKeyShare,
}

#[derive(Clone, Debug)]
pub struct CommitteeSelected {
    pub nodes: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct OutputDecrypted {
    pub output: String,
}

#[derive(Clone, Debug)]
pub struct CiphernodeRegistered {
    pub address: Address,
    // TODO: work out what format public key is and how we should verify
    // signatures
    // pub public_key: PublicKey // <--- BabyJubJub ECPoint?
    pub rank: u32,
    pub new_rank_list_len: u32,
}

#[derive(Clone, Debug)]
pub struct CiphernodeDeregistered {
    pub remove_address: Address,
    pub remove_rank: u32,
    pub last_rank_address: Address,
    pub new_rank_list_lengeth: u32,
}

#[derive(Clone, Debug)]
pub struct ComputationRequested {
    pub e3_id: String,
    // pub computation_type: ??, // TODO:
    // pub execution_model_type: ??, // TODO:
    pub ciphernode_group_length: u32,
    pub ciphernode_threshold: u32,
    // pub input_deadline: ??, // TODO:
    // pub availability_duration: ??, // TODO:
    pub sortition_seed: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EnclaveEventType {
    KeyshareCreated,
    CommitteeSelected,
    OutputDecrypted,
    CiphernodeRegistered,
    CiphernodeDeregistered,
    ComputationRequested,
}

#[derive(Clone, Debug)]
pub enum EnclaveEvent {
    KeyshareCreated(KeyshareCreated),
    CommitteeSelected(CommitteeSelected),
    OutputDecrypted(OutputDecrypted),
    CiphernodeRegistered(CiphernodeRegistered),
    CiphernodeDeregistered(CiphernodeRegistered),
    ComputationRequested(ComputationRequested),
}

impl EnclaveEvent {
    pub fn event_type(&self) -> EnclaveEventType {
        match self {
            EnclaveEvent::KeyshareCreated(_) => EnclaveEventType::KeyshareCreated,
            EnclaveEvent::CommitteeSelected(_) => EnclaveEventType::CommitteeSelected,
            EnclaveEvent::OutputDecrypted(_) => EnclaveEventType::OutputDecrypted,
            EnclaveEvent::CiphernodeRegistered(_) => EnclaveEventType::CiphernodeRegistered,
            EnclaveEvent::CiphernodeDeregistered(_) => EnclaveEventType::CiphernodeDeregistered,
            EnclaveEvent::ComputationRequested(_) => EnclaveEventType::ComputationRequested,
        }
    }
}

#[automock]
#[async_trait]
pub trait EventProducer {
    async fn emit(&self, event: EnclaveEvent) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct FakeProducer;

#[async_trait]
impl EventProducer for FakeProducer {
    async fn emit(&self, _: EnclaveEvent) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
pub trait EventConsumer: Send + Sync + 'static {
    async fn consume(&self, event: EnclaveEvent) -> Result<()>;
}

#[async_trait]
pub trait EventRuntime {
    fn subscribe(&mut self, event_type: EnclaveEventType, handler: Box<dyn EventConsumer>);
    async fn run(&mut self) -> Result<()>;
}

pub struct SimpleEventSubscriber {
    handlers: HashMap<EnclaveEventType, Vec<Box<dyn EventConsumer>>>,
    receiver: mpsc::UnboundedReceiver<EnclaveEvent>,
}

pub struct SimpleEventPublisher {
    sender: mpsc::UnboundedSender<EnclaveEvent>,
}

#[async_trait]
impl EventProducer for SimpleEventPublisher {
    async fn emit(&self, event: EnclaveEvent) -> Result<()> {
        self.sender.send(event)?;
        Ok(())
    }
}

impl SimpleEventSubscriber {
    async fn dispatch(&self, event: EnclaveEvent) -> Result<()> {
        let event_type = event.event_type();

        if let Some(handlers) = self.handlers.get(&event_type) {
            for handler in handlers {
                handler.consume(event.clone()).await?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl EventRuntime for SimpleEventSubscriber {
    fn subscribe(&mut self, event_type: EnclaveEventType, handler: Box<dyn EventConsumer>) {
        self.handlers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    async fn run(&mut self) -> Result<()> {
        while let Some(event) = self.receiver.recv().await {
            self.dispatch(event).await?;
        }
        Ok(())
    }
}


/// Create an event system for use within our test environment
/// In production this will be replaced with libp2p
pub fn create_event_system() -> (SimpleEventPublisher, SimpleEventSubscriber) {
    let (sender, receiver) = mpsc::unbounded_channel();
    (
        SimpleEventPublisher { sender },
        SimpleEventSubscriber {
            handlers: HashMap::new(),
            receiver,
        },
    )
}


