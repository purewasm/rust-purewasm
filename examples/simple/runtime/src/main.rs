use wasmtime::*;

fn main() {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "..\target\wasm32-unknown-unknown\release\purewasm_simple_module.wasm").unwrap();
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let example_func = instance.get_typed_func::<(i64,i64),(i64, i64)>(&mut store, "example").unwrap();
    let alloc_func = instance.get_typed_func::<i64, i64>(&mut store, "alloc").unwrap();

    let len_ptr = alloc_func.call(&mut store, 4).unwrap();
    let ptr = id_func.call(&mut store, len_ptr).unwrap();

    unsafe{
       let mem_slice = std::slice::from_raw_parts(
          memory.data_ptr(&store).offset(ptr as isize), 4);
       println!("Len: {:?}", len_ptr);
       println!("Slice: {:?}", mem_slice);
    }
    
}