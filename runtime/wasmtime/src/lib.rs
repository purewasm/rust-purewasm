mod func;

use func::{get::get_fn, put::put_fn};
use purewasm_runtime::{error::RuntimeError, LedgerStore, WasmBlock, Wasmsg};
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
        let engine = Engine::default();
        let modules = HashMap::new();
        let ledgers = HashMap::new();

        Self {
            engine,
            modules,
            ledgers,
        }
    }

    pub fn add_ledger(&mut self, name: String, ledger: LedgerStoreArc) {
        self.ledgers.insert(name, ledger);
    }

    pub fn add_module(&mut self, name: String, module_binary: &[u8]) -> Result<(), RuntimeError> {
        let module = Module::from_binary(&self.engine, &module_binary)
            .map_err(|e| RuntimeError::Other(format!("{:?}", e)))?;
        self.modules.insert(name, module);
        Ok(())
    }

    pub fn handle(&self, block: WasmBlock) -> Result<(), RuntimeError> {
        let ledger = self
            .ledgers
            .get(&block.ledger)
            .ok_or_else(|| RuntimeError::NoneError)?;
        let block_module = self.modules.get(&block.module).unwrap();
        let pre_msg = Wasmsg {
            method: "pre_handle".to_string(),
            input: block.header.clone(),
        };
        self.handle_message(&block.ledger, block_module, &pre_msg)?;
        for (module_id, messages) in block.messages {
            let module = self.modules.get(&module_id).unwrap();
            for wasmsg in messages {
                self.handle_message(&block.ledger, module, &wasmsg)?
            }
        }
        let post_msg = Wasmsg {
            method: "post_handle".to_string(),
            input: block.header.clone(),
        };
        self.handle_message(&block.ledger, block_module, &post_msg)?;
        ledger.commit().unwrap();
        Ok(())
    }

    fn handle_message(
        &self,
        ledger: &str,
        module: &Module,
        wasmsg: &Wasmsg,
    ) -> Result<(), RuntimeError> {
        let ledger = self
            .ledgers
            .get(ledger)
            .ok_or_else(|| RuntimeError::NoneError)?;
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
            move |caller: Caller<'_, ()>, ptr: i32, len: i32| -> (i32, i32) {
                put_fn(ledger_put.clone(), caller, ptr, len)
            },
        );
        let instance =
            Instance::new(&mut store, &module, &[get_func.into(), put_func.into()]).unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| RuntimeError::NoneError)?;
        let alloc_func = instance.get_typed_func::<i32, i32>(&mut store, "alloc")?;
        let input_len = wasmsg.input.len() as i32;
        let input_ptr = alloc_func.call(&mut store, input_len)?;
        memory
            .write(&mut store, input_ptr as usize, &wasmsg.input)
            .map_err(|_| RuntimeError::Other("".to_string()))?;

        let func = instance.get_typed_func::<(i32, i32), (i32, i32)>(&mut store, &wasmsg.method)?;
        func.call(&mut store, (input_ptr, input_len))?;
        Ok(())
    }
}
