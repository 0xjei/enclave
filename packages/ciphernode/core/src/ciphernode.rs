use std::sync::{Arc, Mutex};

use crate::core::create_and_store_keyshare;
use crate::data::BTreeStore;
use crate::encryptor::AesEncryptor;
use crate::fhe::Fhe;
use crate::{EnclaveEvent, EventConsumer, EventProducer};
use async_trait::*;
use rand::rngs::OsRng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

// Some loose error/result stuff we can use
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Ciphernode instance
#[derive(Clone)]
pub struct Ciphernode<P: EventProducer> {
    moduli: Vec<u64>,
    degree: usize,
    plaintext_modulus: u64,
    rng: Arc<Mutex<ChaCha20Rng>>,
    publisher: P,
}

impl<P: EventProducer> Ciphernode<P> {
    pub async fn create_and_store_keyshare(&self, e3_id: &str) -> Result<()> {
        let fhe = Fhe::new(
            self.rng.clone(),
            self.moduli.clone(),
            self.degree,
            self.plaintext_modulus,
        )?;
        let mut store = BTreeStore::new();
        let key = b"a 32-byte secret key here!!!!!!!".to_vec();
        let encryptor = AesEncryptor::new(key);
        create_and_store_keyshare(
            &fhe,
            self.rng.clone(),
            &mut store,
            &encryptor,
            &self.publisher,
            &e3_id,
        )
        .await?;

        Ok(())
    }

    pub fn new(publisher: P) -> Result<Self> {
        Ok(Ciphernode {
            moduli: vec![0x3FFFFFFF000001],
            degree: 2048,
            plaintext_modulus: 1032193,
            rng: Arc::new(Mutex::new(ChaCha20Rng::from_rng(OsRng)?)),
            publisher,
        })
    }
}

#[async_trait]
impl<P: EventProducer> EventConsumer for Ciphernode<P> {
    async fn consume(&self, event: EnclaveEvent) -> Result<()> {
        Ok(match event {
            EnclaveEvent::ComputationRequested(c) => {
                self.create_and_store_keyshare(&c.e3_id).await?
            }
            _ => (),
        })
    }
}
