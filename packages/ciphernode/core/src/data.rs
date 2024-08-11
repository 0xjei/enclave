use std::collections::BTreeMap;

use mockall::automock;

// Some loose error/result stuff we can use
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[automock]
pub trait Store {
    fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>>;
    // fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
}

pub struct BTreeStore {
    map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl BTreeStore {
    pub fn new() -> Self {
        BTreeStore {
            map: BTreeMap::new(),
        }
    }
}

impl Store for BTreeStore {
    fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.map.insert(key.to_vec(), value.to_vec()))
    }
}
