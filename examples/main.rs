use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use purewasm_cbor::CborCodec;
use purewasm_core::codec::Codec;
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
    }
}
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
    let codec = CborCodec {};
    let input = codec.to_bytes(&input).unwrap();
    let msg = purewasm_runtime::Wasmsg {
        method: "create".to_string(),
        input: input,
    };
    let dinput = DeleteUserInput {
        username: "ademcaglin".to_string(),
        deleted_at: 45
    };
    let dinput = codec.to_bytes(&dinput).unwrap();
    let dmsg = purewasm_runtime::Wasmsg {
        method: "delete".to_string(),
        input: dinput,
    };
    block.messages.insert("test_module".to_string(), vec![msg, dmsg]);
    runtime.handle(block).unwrap();

    //let r: Result<CreateUserInput, WasmError> = CborCodec::from_bytes(&r);
    let state = store.state.lock().unwrap();
    let user: User = CborCodec::from_bytes(state.get("/users/ademcaglin").unwrap()).unwrap();
    println!("User: {:?}", user);
}
