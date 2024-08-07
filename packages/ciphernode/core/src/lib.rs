#![crate_name = "core"]
#![crate_type = "lib"]
#![warn(missing_docs, unused_imports)]

mod fhe;
mod core;
mod encryptor;
mod data;
mod events;

pub struct Core {
	pub name: String,
}

impl Core {
    fn new(name: String) -> Self {
        Self { name }
    }
}

