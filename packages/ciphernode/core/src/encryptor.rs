use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key,
};
use mockall::automock;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// Some loose error/result stuff we can use
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[automock]
pub trait Encryptor<T> {
    fn encrypt(&self, data: T) -> Result<Encrypted<T>>;
}

#[automock]
pub trait Decryptor<T> {
    fn decrypt(&self, data: Encrypted<T>) -> Result<T>;
}

#[derive(Debug, PartialEq)]
pub struct Encrypted<T> {
    ciphertext: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T> Encrypted<T> {
    pub fn new(ciphertext: Vec<u8>) -> Encrypted<T> {
        Encrypted {
            ciphertext,
            _phantom: PhantomData,
        }
    }
}

// Manually implementing clone so we don't
// need to implement it on SecretKey
impl<T> Clone for Encrypted<T> {
    fn clone(&self) -> Self {
        Encrypted {
            ciphertext: self.ciphertext.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Encrypted<T> {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
}

// This in an attempt to implement an AesEncryptor for the db data to sanity check the interfaces
// TODO: decryption as required
pub struct AesEncryptor {
    key: Vec<u8>,
}

impl AesEncryptor {
    pub fn new(key: Vec<u8>) -> AesEncryptor {
        AesEncryptor { key }
    }
}

impl<T: Into<Vec<u8>>> Encryptor<T> for AesEncryptor {
    fn encrypt(&self, data: T) -> Result<Encrypted<T>> {
        let serialized: Vec<u8> = data.into();
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.key));
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, serialized.as_ref())
            .expect("Encryption failed"); // TODO: fix this when tidying up errors
        Ok(Encrypted {
            ciphertext,
            _phantom: std::marker::PhantomData,
        })
    }
}
