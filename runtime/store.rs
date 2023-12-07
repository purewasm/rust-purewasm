use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmDocument {
    value: Option<Vec<u8>>,
    events: Vec<Vec<u8>>,
}

pub trait Store {
    fn get(&self, key: &str) -> Result<WasmDocument, String>;
    fn set_value(&self, key: &str, value: &[u8]) -> Result<(), String>;
    fn push_event(&self, key: &str, event: &[u8]) -> Result<(), String>;
}
