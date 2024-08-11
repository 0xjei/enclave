#![crate_name = "core"]
#![crate_type = "lib"]
#![warn(missing_docs, unused_imports)]

use core::create_and_store_keyshare;

use data::BTreeStore;
use encryptor::AesEncryptor;
use events::FakeProducer;
use fhe::Fhe;
use rand::rngs::ThreadRng;

mod core;
mod data;
mod encryptor;
mod events;
mod fhe;

// Some loose error/result stuff we can use
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Ciphernode instance
pub struct Ciphernode {
    fhe: Fhe,
    rng: ThreadRng,
    store: BTreeStore,
    encryptor: AesEncryptor,
    producer: FakeProducer,
}

impl Ciphernode {
    pub async fn create_and_store_keyshare(&mut self, e3_id: &str) -> Result<()> {
        create_and_store_keyshare(
            &self.fhe,
            &mut self.rng,
            &mut self.store,
            &self.encryptor,
            &self.producer,
            &e3_id,
        )
        .await?;

        Ok(())
    }

    pub fn new() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let fhe = Fhe::new(&mut rng, vec![0x3FFFFFFF000001], 2048, 1032193)?;
        let store = BTreeStore::new();
        let encryptor = AesEncryptor::new(b"this is a secret".to_vec());
        let producer = FakeProducer {};

        Ok(Ciphernode {
            fhe,
            rng,
            store,
            encryptor,
            producer,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Ciphernode;

    // Some loose error/result stuff we can use
    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;

    #[tokio::test]
    #[ignore]
    async fn test_ciphernode() -> Result<()> {
        unimplemented!();
    }
}


