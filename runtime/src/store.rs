use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

pub trait Store {
    fn get(&self, key: &str) -> Result<Vec<u8>, String>;
    fn put(&self, key: &str, value: &[u8]) -> Result<(), String>;
}
