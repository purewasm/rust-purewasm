use crate::codec::Codec;
use crate::error::WasmError;
use serde::{de::DeserializeOwned, Serialize};

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
        // 'to_vec' will copy the data because slice might cause memory leak
        let bytes = core::slice::from_raw_parts(ptr, len as usize).to_vec();
        C::from_bytes(&bytes)
    }
}
