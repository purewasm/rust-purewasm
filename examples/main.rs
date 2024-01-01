use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use purewasm_runtime::{error::RuntimeError, LedgerStore};
use purewasm_runtime_wasmtime::WasmRuntime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub name: String,
    pub birthday: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserInput {
    pub username: String,
    pub deleted_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub name: String,
    pub birthday: u32,
    pub events: Vec<UserEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub name: String,
    pub birthday: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeleteddEvent {
    pub at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserEvent {
    Created(UserCreatedEvent),
    Deleted(UserDeleteddEvent),
}
pub struct InMemoryStore {
    state: Mutex<HashMap<String, Vec<u8>>>,
}

impl LedgerStore for InMemoryStore {
    fn get(&self, key: &str) -> Vec<u8> {
        let state = self.state.lock().unwrap();
        if let Some(value) = state.get(key) {
            return value.to_owned();
        }
        vec![]
    }

    fn put(&self, key: &str, value: &[u8]) -> Result<(), RuntimeError> {
        let mut state = self.state.lock().unwrap();
        state.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn commit(&self) -> Result<(), RuntimeError> {
        Ok(())
    }}
fn main() {
    let mut runtime = WasmRuntime::new();
    let store = Arc::new(InMemoryStore {
        state: Mutex::new(HashMap::new()),
    });
    runtime.add_ledger("test", store.clone());
    runtime
        .add_module(
            "test_module",
            include_bytes!("../target/wasm32-unknown-unknown/debug/purewasm_module.wasm"),
        )
        .unwrap();
    let mut block = purewasm_runtime::WasmBlock {
        ledger: "test".to_string(),
        module: "test".to_string(),
        header: vec![],
        messages: HashMap::new(),
    };
    let input = CreateUserInput {
        username: "ademcaglin".to_string(),
        name: "adem caglin".to_string(),
        birthday: 145,
    };
    let mut input_bytes: Vec<u8> = vec![];
    ciborium::into_writer(&input, &mut input_bytes).unwrap();
    let msg = purewasm_runtime::Wasmsg {
        method: "create".to_string(),
        input: input_bytes,
        proof: vec![]
    };
    let dinput = DeleteUserInput {
        username: "ademcaglin".to_string(),
        deleted_at: 45,
    };
    let mut dinput_bytes: Vec<u8> = vec![];
    ciborium::into_writer(&dinput, &mut dinput_bytes).unwrap();
    let dmsg = purewasm_runtime::Wasmsg {
        method: "delete".to_string(),
        input: dinput_bytes,
        proof: vec![]
    };
    block.messages.insert("test_module".to_string(), vec![msg, dmsg]);
    runtime.handle(block).unwrap();

    //let r: Result<CeateUserInput, WasmError> = CborCodec::from_bytes(&r);
    let state = store.state.lock().unwrap();
    let key = state.get("/users/ademcaglin").unwrap();
    let user: User = ciborium::from_reader(key.as_slice()).unwrap();
    println!("User: {:?}", user);
}
