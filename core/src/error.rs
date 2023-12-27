use alloc::string::String;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum WasmError {
    NotAuthorized,
    NotFound,
    NoInput, 
    SerializeError,
    DeserializeError,
    HostError,
    Other(String),
}
