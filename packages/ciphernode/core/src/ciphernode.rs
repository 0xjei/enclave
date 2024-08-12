use crate::core::create_and_store_keyshare;
use crate::data::BTreeStore;
use crate::encryptor::AesEncryptor;
use crate::events::FakeProducer;
use crate::fhe::Fhe;
use rand::rngs::ThreadRng;

// Some loose error/result stuff we can use
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;


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

