#![crate_name = "enclave_core"]
#![crate_type = "lib"]
// commenting out for now
// #![warn(missing_docs, unused_imports)]

mod ciphernode;
mod core;
mod data;
mod encryptor;
mod events;
mod fhe;

pub use ciphernode::*;
pub use events::*;

#[cfg(test)]
mod tests {

    // Some loose error/result stuff we can use
    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;

    #[tokio::test]
    #[ignore]
    async fn test_ciphernode() -> Result<()> {
        unimplemented!();
    }
}
