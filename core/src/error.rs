use alloc::string::{String, ToString };
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct  WasmError {
    pub code: String,
    pub message: Option<String>,
}

impl WasmError {
    pub fn code(code: &str) -> Self {
        WasmError {
            code: code.to_string(),
            message: None,
        }
    }
    pub fn new(code: &str, msg: &str) -> Self {
        WasmError {
            code: code.to_string(),
            message: Some(msg.to_string()),
        }
    }
}
