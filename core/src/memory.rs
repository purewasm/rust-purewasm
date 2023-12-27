use serde::{de::DeserializeOwned, Serialize};

use crate::{codec::Codec, error::WasmError};

pub struct WasmMemory<C: Codec> {
    pub codec: C,
}

impl<C: Codec> WasmMemory<C> {
    pub fn to_memory<T: Serialize>(&self, t: T) -> Result<(i32, i32), WasmError> {
        let bytes = self.codec.to_bytes(&t)?;
        let output_ptr = bytes.as_ptr();
        let output_len = bytes.len() as i32;
        Ok((output_ptr as i32, output_len))
    }

    pub unsafe fn from_memory<T: DeserializeOwned>(
        &self,
        ptr: *mut u8,
        len: i32,
    ) -> Result<T, WasmError> {
        let bytes = core::slice::from_raw_parts(ptr, len as usize);
        C::from_bytes(bytes)
    }
}
