use purewasm_codec::Codec;
use purewasm_model::PureError;
use serde::{de::DeserializeOwned, Serialize};

pub struct WasmMemory<C: Codec> {
    pub codec: C,
}

impl<C: Codec> WasmMemory<C> {
    pub fn to_memory<T: Serialize>(&self, t: T) -> (i32, i32) {
        let r = self.codec.to_bytes(&t);
        let bytes = match r {
            Ok(bytes) => bytes,
            Err(e) => self.codec.to_bytes(&e).unwrap(),
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
#[cfg(test)]
mod tests {
    use purewasm_codec::cbor::CborCodec;

    use super::*;

    #[test]
    fn ptr_test() {
        let bytes: [u8; 5] = [1, 2, 3, 4, 5];

        // Get the pointer as an i32
        let ptr = bytes.as_ptr() as i32;

        // Convert the i32 pointer back to a reference
        let original_slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, 5) };

        // Now you can work with the original slice
        for &byte in original_slice {
            println!("Byte: {}", byte);
        }
    }
    #[test]
    fn test_memory() {
        let codec = CborCodec;
        let wasm_memory = WasmMemory { codec };

        let input = 42;
        let (output_ptr, output_len) = wasm_memory.to_memory(input);
        eprintln!("{:?}", output_ptr);
        unsafe {
            eprintln!("{:?}", (output_ptr as *mut u8).as_ref().unwrap());
        }

        /*let r: i32 = unsafe {
            wasm_memory
                .from_memory(output_ptr as *mut u8, output_len)
                .unwrap()
        };*/
    }
}
/*
pub struct MockCodec;

    impl Codec for MockCodec {
        fn to_bytes<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, PureError> {
            unsafe {
                Ok(core::slice::from_raw_parts(
                    (&value as *const T) as *const u8,
                    ::core::mem::size_of::<T>(),
                )
                .to_vec())
            }
        }

        fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PureError> {
            let t: T = unsafe { std::ptr::read(bytes.as_ptr() as *const _) };

            Ok(t)
        }

        fn get_code(&self) -> i64 {
            todo!()
        }
    }
*/
