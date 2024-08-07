use mockall::automock;

// Some loose error/result stuff we can use
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[automock]
pub trait Store {
    fn insert(&self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>>;
    // fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
}

