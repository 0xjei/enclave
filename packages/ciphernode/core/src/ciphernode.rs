use crate::core::create_and_store_keyshare;
use crate::data::BTreeStore;
use crate::encryptor::AesEncryptor;
use crate::events::FakeProducer;
use crate::fhe::Fhe;
use crate::EventConsumer;
use async_trait::*;

// Some loose error/result stuff we can use
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// Ciphernode instance
pub struct Ciphernode {
    moduli: Vec<u64>,
    degree: usize,
    plaintext_modulus: u64,
}

impl Ciphernode {
    pub async fn create_and_store_keyshare(&mut self, e3_id: &str) -> Result<()> {
        let mut rng = rand::thread_rng();
        let fhe = Fhe::new(
            &mut rng,
            self.moduli.clone(),
            self.degree,
            self.plaintext_modulus,
        )?;
        let mut store = BTreeStore::new();
        let encryptor = AesEncryptor::new(b"this is a secret".to_vec());
        let producer = FakeProducer {};
        create_and_store_keyshare(&fhe, &mut rng, &mut store, &encryptor, &producer, &e3_id)
            .await?;

        Ok(())
    }

    pub fn new() -> Result<Self> {
        Ok(Ciphernode {
            moduli: vec![0x3FFFFFFF000001],
            degree: 2048,
            plaintext_modulus: 1032193,
        })
    }
}

#[async_trait]
impl EventConsumer for Ciphernode {
    async fn consume(&self, event: crate::EnclaveEvent) -> Result<()> {
        match event {
            _ => println!("Event received!"),
        }
        Ok(())
    }
}
