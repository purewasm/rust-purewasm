use std::{collections::HashMap, sync::Mutex};

use purewasm_runtime::{error::RuntimeError, LedgerStore};

struct InMemoryStore {
    state: Mutex<HashMap<String, Vec<u8>>>,
}

impl LedgerStore for InMemoryStore {
    fn get(&self, key: &str) -> Option<Vec<u8>> {
        let state = self.state.lock().unwrap();
        let value = state.get(key)?;
        Some(value.to_owned())
    }

    fn put(&self, key: &str, value: &[u8]) -> Result<(), RuntimeError> {
        let mut state = self.state.lock().unwrap();
        state.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn commit(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
}
fn main() {
    println!("Hello, world!");
}
