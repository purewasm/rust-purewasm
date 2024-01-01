use std::collections::HashMap;

use error::RuntimeError;

pub mod error;

#[derive(Clone, Debug)]
pub struct WasmBlock {
    pub ledger: String,
    pub module: String,
    pub header: Vec<u8>,
    pub messages: HashMap<String, Vec<Wasmsg>>,
}

#[derive(Clone, Debug)]
pub struct Wasmsg {
    pub method: String,
    pub input: Vec<u8>,
    pub proof: Vec<u8>,
}

pub trait LedgerStore {
    fn get(&self, key: &str) -> Vec<u8>;
    fn put(&self, key: &str, value: &[u8]) -> Result<(), RuntimeError>;
    fn commit(&self) -> Result<(), RuntimeError>;
}
