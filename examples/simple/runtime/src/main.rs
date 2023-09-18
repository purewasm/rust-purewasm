use wasmtime::*;
use purewasm_core::Codec;
use purewasm_simple_shared::{Input, CustomResult};

fn main() {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "../target/wasm32-unknown-unknown/release/purewasm_simple_module.wasm").unwrap();
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let codec =  purewasm_codec::cbor::CborCodec;
    let input = purewasm_simple_shared::Input{
        code: 6
    };
    let input_bytes: Vec<u8> = codec.to_bytes(&input).unwrap();
    let input_bytes_len = input_bytes.len() as i32;
    let alloc_func = instance.get_typed_func::<i32, i32>(&mut store, "alloc").unwrap();
    let input_bytes_ptr = alloc_func.call(&mut store, input_bytes_len).unwrap();
    memory.write(&mut store, input_bytes_ptr as usize, &input_bytes).unwrap();

    let example_func = instance.get_typed_func::<(i32, i32), (i32,i32)>(&mut store, "example").unwrap();
    let (result_ptr, result_len) = example_func.call(&mut store, (input_bytes_ptr,input_bytes_len)).unwrap();
    println!("ptr: {:?}, len: {}", result_ptr, result_len);

    unsafe{
       let mem_slice = std::slice::from_raw_parts(
          memory.data_ptr(&store).offset(result_ptr as isize), result_len as usize);
       /*let r: purewasm_simple_shared::CustomResult = 
         purewasm_core::codec::cbor::CborCodec::from_bytes(mem_slice).unwrap();*/
         let r: purewasm_core::PureResult<purewasm_simple_shared::CustomResult> = 
         purewasm_codec::cbor::CborCodec::from_bytes(mem_slice);
       //println!("Len: {:?}", len_ptr);
       println!("Slice: {:?}", r);
    }
    
}