use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::{CodecImpl, WasmError, WasmMemory};
extern "C" {
    pub fn get(key_ptr: i32, key_len: i32) -> (i32, i32);
    pub fn put(key_ptr: i32, key_len: i32, value_ptr: i32, value_len: i32);
    pub fn get_signers() -> (i32, i32);
    pub fn log_error(ptr: i32, len: i32);
}

pub trait Host {
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, WasmError>;
    fn put<T: Serialize>(&self, key: &str, value: &T) -> Result<(), WasmError>;
    fn get_signers(&self) -> Result<Vec<String>, WasmError>;
    fn log_error(&self, msg: &str);
}

pub struct HostImpl;

impl Host for HostImpl {
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, WasmError> {
        let result = unsafe { get(key.as_ptr() as i32, key.len() as i32) };
        if result == (0, 0) {
            return Ok(None);
        }
        let memory = WasmMemory {
            codec: CodecImpl {},
        };
        let value: T = unsafe { memory.from_memory(result.0 as *mut u8, result.1)? };
        Ok(Some(value))
    }

    fn put<T: Serialize>(&self, key: &str, value: &T) -> Result<(), WasmError> {
        let memory = WasmMemory {
            codec: CodecImpl {},
        };
        let (value_ptr, value_len) = memory.to_memory(value)?;
        unsafe {
            put(key.as_ptr() as i32, key.len() as i32, value_ptr, value_len);
        }
        Ok(())
    }

    fn get_signers(&self) -> Result<Vec<String>, WasmError> {
        unsafe {
            let result = get_signers();
            let bytes = core::slice::from_raw_parts(result.0 as *mut u8, result.1 as usize);
            let signers = String::from_utf8(bytes.to_vec()).map_err(|e| WasmError::code("UTF8"))?;
            Ok(signers
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
        }
    }

    fn log_error(&self, msg: &str) {
        unsafe {
            log_error(msg.as_ptr() as i32, msg.as_bytes().len() as i32);
        }
    }
}
