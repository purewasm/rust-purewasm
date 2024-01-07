mod error;
mod func;
mod store;

use crate::func::{get::get_fn, put::put_fn};
use crate::{error::RuntimeError, store::LedgerStore};
use std::{collections::HashMap, sync::Arc};
use wasmtime::*;

type LedgerStoreArc = Arc<dyn LedgerStore + Send + Sync + 'static>;

pub struct WasmRuntime {
    engine: Engine,
    modules: HashMap<String, Module>,
    ledgers: HashMap<String, LedgerStoreArc>,
}

impl WasmRuntime {
    pub fn new() -> Self {
        let engine = Engine::new(Config::new().debug_info(true)).unwrap();
        let modules = HashMap::new();
        let ledgers = HashMap::new();

        Self {
            engine,
            modules,
            ledgers,
        }
    }

    pub fn add_ledger(&mut self, name: &str, ledger: LedgerStoreArc) {
        self.ledgers.insert(name.to_string(), ledger);
    }

    pub fn add_module(&mut self, name: &str, module_binary: &[u8]) -> Result<(), RuntimeError> {
        let module = Module::from_binary(&self.engine, &module_binary)?;
        self.modules.insert(name.to_string(), module);
        Ok(())
    }

    /*pub fn handle(&self, block: WasmBlock) -> Result<(), RuntimeError> {
        let ledger = self
            .ledgers
            .get(&block.ledger)
            .ok_or_else(|| RuntimeError::NoneError)?;
        /*let block_module = self.modules.get(&block.module).unwrap();
        let pre_msg = Wasmsg {
            method: "pre_handle".to_string(),
            input: block.header.clone(),
        };
        self.handle_message(&block.ledger, block_module, &pre_msg)?;*/
        for (module_id, messages) in block.messages {
            let module = self.modules.get(&module_id).unwrap();
            for wasmsg in messages {
                self.handle_message(&block.ledger, module, &wasmsg)?;
            }
        }
        /*let post_msg = Wasmsg {
            method: "post_handle".to_string(),
            input: block.header.clone(),
        };
        self.handle_message(&block.ledger, block_module, &post_msg)?;*/
        ledger.commit().unwrap();
        Ok(())
    }*/

    fn handle_message(
        &self,
        ledger: &str,
        module: &Module,
        wasmsg: &[u8],
    ) -> Result<(), RuntimeError> {
        let ledger = self
            .ledgers
            .get(ledger)
            .ok_or_else(|| RuntimeError::NoneError)?;
        let mut store = Store::new(&self.engine, ());

        let ledger_get = ledger.clone();
        let ledger_put = ledger.clone();
        let get_func = Func::wrap(
            &mut store,
            move |caller: Caller<'_, ()>, ptr: i32, len: i32| -> (i32, i32) {
                get_fn(ledger_get.clone(), caller, ptr, len)
            },
        );

        let put_func = Func::wrap(
            &mut store,
            move |caller: Caller<'_, ()>,
                  key_ptr: i32,
                  key_len: i32,
                  value_ptr: i32,
                  value_len: i32| {
                put_fn(
                    ledger_put.clone(),
                    caller,
                    key_ptr,
                    key_len,
                    value_ptr,
                    value_len,
                )
            },
        );
        let instance = Instance::new(&mut store, &module, &[get_func.into(), put_func.into()])?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(||RuntimeError::NoneError)?;
        let alloc_func = instance.get_typed_func::<i32, i32>(&mut store, "alloc")?;
        let de_alloc_func = instance.get_typed_func::<i32, ()>(&mut store, "de_alloc")?;

        let input_bytes = wasmsg.clone();
        let input_bytes_len = input_bytes.len() as i32;
        let input_bytes_ptr = alloc_func.call(&mut store, input_bytes_len)?;
        memory
            .write(&mut store, input_bytes_ptr as usize, &input_bytes)
            .unwrap();

        let func = instance.get_typed_func::<(i32, i32), (i32, i32)>(&mut store, "handle")?;
        let result = func.call(&mut store, (input_bytes_ptr, input_bytes_len))?;
        de_alloc_func.call(&mut store, result.0)?;

        //unsafe { drop(Box::from_raw(result.0 as *mut u8)) };
        println!("Result: {:?}", result);
        Ok(())
    }
}
