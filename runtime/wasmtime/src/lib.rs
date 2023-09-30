use purewasm_core::{Codec, PureResult};
use wasmtime::{Engine, Instance, Module, Store};

#[cfg(feature = "cbor")]
pub use purewasm_cbor as cbor;
#[cfg(feature = "json")]
pub use purewasm_json as json;

pub struct PureModule<T: Codec> {
    pub file: String,
    pub codec: T,
}

impl<T: Codec> PureModule<T> {
    pub fn call_fn<I: serde::Serialize, R: serde::de::DeserializeOwned>(&self, fn_name: &str, input: I) -> PureResult<R> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, &self.file).unwrap();
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();
        let memory = instance.get_memory(&mut store, "memory").unwrap();
        let input_bytes: Vec<u8> = self.codec.to_bytes(&input).unwrap();
        let input_bytes_len = input_bytes.len() as i32;
        let alloc_func = instance
            .get_typed_func::<i32, i32>(&mut store, "alloc")
            .unwrap();
        let input_bytes_ptr = alloc_func.call(&mut store, input_bytes_len).unwrap();
        memory
            .write(&mut store, input_bytes_ptr as usize, &input_bytes)
            .unwrap();

        let func = instance
            .get_typed_func::<(i32, i32), (i32, i32)>(&mut store, fn_name)
            .unwrap();
        let (result_ptr, result_len) = func
            .call(&mut store, (input_bytes_ptr, input_bytes_len))
            .unwrap();
        unsafe {
            let mem_slice = std::slice::from_raw_parts(
                memory.data_ptr(&store).offset(result_ptr as isize),
                result_len as usize,
            );
            let result: purewasm_core::PureResult<R> =
               T::from_bytes(mem_slice).unwrap();
            return result;
        }
    }
}
