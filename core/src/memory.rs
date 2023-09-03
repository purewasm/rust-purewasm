use serde::{de::DeserializeOwned, Serialize};
use crate::{codec::Codec, error::PureError};

pub struct WasmMemory<C: Codec> {
    pub codec: C,
}

impl<C: Codec> WasmMemory<C> {
    pub fn to_memory<T: Serialize>(&self, t: T) -> (i64, i64) {
        let r = self.codec.to_bytes(t);
        let bytes = match r {
            Ok(bytes) => bytes,
            Err(e) => self.codec.to_bytes(e).unwrap(),
        };
        let output_ptr = bytes.as_ptr();
        let output_len = bytes.len() as i64;
        (output_ptr as i64, output_len)
    }

    pub unsafe fn from_memory<T: DeserializeOwned>(
        &self,
        ptr: *mut u8,
        len: i64,
    ) -> Result<T, PureError> {
        let bytes = core::slice::from_raw_parts(ptr, len as usize);
        C::from_bytes(bytes)
    }
}
