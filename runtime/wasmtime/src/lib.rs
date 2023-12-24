use func::{get::get_fn, put::put_fn};
use std::{collections::HashMap, sync::Arc};
use wasmtime::*;
mod error;
mod func;

#[derive(Clone, Debug)]
pub struct WasmBlock {
    ledger: String,
    messages: HashMap<String, Vec<Wasmsg>>,
}

#[derive(Clone, Debug)]
pub struct Wasmsg {
    method: String,
    input: Vec<u8>,
}

pub trait KvStore {
    fn get(&self, key: &str) -> Result<Vec<u8>, String>;
    fn put(&self, key: &str, value: &[u8]) -> Result<(), String>;
    fn get_events(&self, key: &str) -> Result<Vec<Vec<u8>>, String>;
    fn push_event(&self, key: &str, event: &[u8]) -> Result<(), String>;
    fn commit(&self) -> Result<(), String>;
}

pub struct WasmRuntime {
    engine: Engine,
    modules: HashMap<String, Module>,
    ledgers: HashMap<String, Arc<dyn KvStore + Send + Sync + 'static>>,
}

impl WasmRuntime {
    pub fn new() -> Self {
        let engine = Engine::default();
        let modules = HashMap::new();
        let ledgers = HashMap::new();

        Self {
            engine,
            modules,
            ledgers,
        }
    }

    pub fn add_ledger(&mut self, name: String, ledger: Arc<dyn KvStore + Send + Sync + 'static>) {
        self.ledgers.insert(name, ledger);
    }

    pub fn add_module(&mut self, name: String, module_binary: &[u8]) {
        let module = Module::from_binary(&self.engine, &module_binary).unwrap();
        self.modules.insert(name, module);
    }

    pub fn handle(&self, block: WasmBlock) -> Result<(), String> {
        let ledger: Arc<dyn KvStore + Send + Sync + 'static> =
            self.ledgers.get(&block.ledger).unwrap().clone();
        let ledger_get = ledger.clone();
        let ledger_put = ledger.clone();

        let mut store = Store::new(&self.engine, ());
        let get_func = Func::wrap(
            &mut store,
            move |caller: Caller<'_, ()>, ptr: i32, len: i32| -> (i32, i32) {
                get_fn(ledger_get.clone(), caller, ptr, len)
            },
        );

        let put_func = Func::wrap(
            &mut store,
            move |caller: Caller<'_, ()>, ptr: i32, len: i32| -> (i32, i32) { put_fn(ledger_put.clone(), caller, ptr, len) },
        );

        for (module_id, messages) in block.messages {
            let module = self.modules.get(&module_id).unwrap();
            for wasmsg in messages {
                let instance =
                    Instance::new(&mut store, &module, &[get_func.into(), put_func.into()])
                        .unwrap();
                let memory = instance.get_memory(&mut store, "memory").unwrap();
                let alloc_func = instance
                    .get_typed_func::<i32, i32>(&mut store, "alloc")
                    .unwrap();
                let input_len = wasmsg.input.len() as i32;
                let input_ptr = alloc_func.call(&mut store, input_len).unwrap();
                memory
                    .write(&mut store, input_ptr as usize, &wasmsg.input)
                    .unwrap();

                let func = instance
                    .get_typed_func::<(i32, i32), (i32, i32)>(&mut store, &wasmsg.method)
                    .unwrap();
                let _ = func.call(&mut store, (input_ptr, input_len)).unwrap();
            }
        }
        ledger.commit().unwrap();        
        Ok(())
    }
}
