use wasmtime::{Engine, Instance, Memory, Module, Store, Caller, Extern, TypedFunc, Func};

pub trait KvStore {
    fn get(&self, key: &str) -> Result<Vec<u8>, String>;
    fn put(&self, key: &str, value: &[u8]) -> Result<(), String>;
}
pub struct WasmRuntime<S: KvStore> {
    kv_store: S,
}

pub struct PureModule {
    instance: Instance,
    store: Store<()>,
    memory: Memory,
}

impl PureModule {
    pub fn from_binary(binary: &[u8]) -> Self {
        let engine = Engine::default();
        let module = Module::from_binary(&engine, binary).unwrap();
        Self::new(module, engine)
    }
    pub fn from_file(file: &str) -> Self {
        let engine = Engine::default();
        let module = Module::from_file(&engine, file).unwrap();
        Self::new(module, engine)
    }

    pub fn new(module: Module, engine: Engine) -> Self {
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();
        let memory = instance.get_memory(&mut store, "memory").unwrap();

        Self {
            instance,
            store,
            memory,
        }
    }

    pub fn call_fn(&mut self, fn_name: &str, input_bytes: &[u8]) -> Vec<u8> {
        let input_bytes_len = input_bytes.len() as i32;
        let alloc_func = self
            .instance
            .get_typed_func::<i32, i32>(&mut self.store, "alloc")
            .unwrap();
        let input_bytes_ptr = alloc_func.call(&mut self.store, input_bytes_len).unwrap();
        self.memory
            .write(&mut self.store, input_bytes_ptr as usize, &input_bytes)
            .unwrap();

        let func = self
            .instance
            .get_typed_func::<(i32, i32), (i32, i32)>(&mut self.store, fn_name)
            .unwrap();
        let (result_ptr, result_len) = func
            .call(&mut self.store, (input_bytes_ptr, input_bytes_len))
            .unwrap();
        unsafe {
            let mem_slice = std::slice::from_raw_parts(
                self.memory
                    .data_ptr(&self.store)
                    .offset(result_ptr as isize),
                result_len as usize,
            );
            mem_slice.to_vec()
        }
    }
}

fn get_func() {
    let engine = Engine::default();

    let mut store = Store::new(&engine, 4);

    let get = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, u32>, ptr: i32, len: i32| -> (i32, i32) {
            let memory = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => panic!("`memory` export not found"),
            };
            let func = match caller.get_export("alloc") {
                Some(Extern::Func(func)) => func,
                _ => panic!("`alloc` export not found"),
            };
            let data = memory
                .data(&caller)
                .get(ptr as u32 as usize..)
                .and_then(|arr| arr.get(..len as u32 as usize));
            let input_bytes = vec![1,2,3,4];

           let input_bytes_len = input_bytes.len() as i32;
            let alloc_func: TypedFunc<i32, i32> = func
                .typed::<i32, i32>(&store)
                .unwrap();
             let input_bytes_ptr = alloc_func.call(&mut store, input_bytes_len).unwrap();
            memory
                .write(&mut store, input_bytes_ptr as usize, &input_bytes)
                .unwrap();
            /*let key = unsafe {
                String::from_raw_parts(
                    memory.data_ptr(&store).offset(ptr as isize),
                    len as usize,
                    len as usize,
                )
            };*/

            println!(
                "Got {:?} from WebAssembly",
                std::str::from_utf8(data.unwrap()).unwrap()
            );
            //let key = String::from("my_result");
            println!("my host state is: {}", caller.data());
            (5, 5)
        },
    );
    //get
}
