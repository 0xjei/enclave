use mockall::automock;

use crate::fhe::PublicKeyShare;

// Some loose error/result stuff we can use
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub enum EnclaveEvent {
    PublicKeyshareCreated(PublicKeyShare),
    OutputDecrypted(String),
}

#[automock]
pub trait EventProducer {
    fn emit(&self, event: EnclaveEvent) -> Result<()>;
}
