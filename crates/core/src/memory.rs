use crate::codec::Codec;
use crate::error::PureError;
use crate::serde::{de::DeserializeOwned, Serialize};

pub struct WasmMemory<C: Codec> {
    pub codec: C,
}

impl<C: Codec> WasmMemory<C> {
    pub fn to_memory<T: Serialize>(&self, t: T) -> (i32, i32) {
        let r = self.codec.to_bytes(t);
        let bytes = match r {
            Ok(bytes) => bytes,
            Err(e) => self.codec.to_bytes(e).unwrap(),
        };
        let output_ptr = bytes.as_ptr();
        let output_len = bytes.len() as i32;
        (output_ptr as i32, output_len)
    }

    pub unsafe fn from_memory<T: DeserializeOwned>(
        &self,
        ptr: *mut u8,
        len: i32,
    ) -> Result<T, PureError> {
        let bytes = core::slice::from_raw_parts(ptr, len as usize);
        C::from_bytes(bytes)
    }
}
