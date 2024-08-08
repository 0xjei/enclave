use mockall::automock;

use crate::fhe::PublicKeyShare;

// Some loose error/result stuff we can use
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct KeyshareCreated {
    pub pubkey: PublicKeyShare,
}

pub struct CommitteeSelected {
    pub nodes: Vec<String>,
}

pub struct OutputDecrypted {
    pub output: String,
}

pub enum EnclaveEvent {
    KeyshareCreated(KeyshareCreated),
    CommitteeSelected(CommitteeSelected),
    OutputDecrypted(OutputDecrypted),
}

#[automock]
pub trait EventProducer {
    fn emit(&self, event: EnclaveEvent) -> Result<()>;
}

#[automock]
pub trait EventConsumer {
    fn consume(&mut self, event: &EnclaveEvent);
}
