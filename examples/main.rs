use std::{sync::Mutex, collections::HashMap};

/*use purewasm_runtime::{LedgerStore, error::RuntimeError};

struct InMemoryStore {
   state: Mutex<HashMap<String, Vec<u8>>>
}

impl LedgerStore for InMemoryStore {
    fn get(&self, key: &str) -> Result<Vec<u8>, RuntimeError> {
        todo!()
    }

    fn put(&self, key: &str, value: &[u8]) -> Result<(), RuntimeError> {
        todo!()
    }

    fn get_events(&self, key: &str) -> Result<Vec<Vec<u8>>, RuntimeError> {
        todo!()
    }

    fn push_event(&self, key: &str, event: &[u8]) -> Result<(), RuntimeError> {
        todo!()
    }

    fn commit(&self) -> Result<(), RuntimeError> {
        todo!()
    }
}*/
fn main() {
    println!("Hello, world!");
}