use crate::LedgerStore;
use std::sync::Arc;
use wasmtime::{Caller, Extern, Memory};

pub fn put_fn(
    ledger_store: Arc<dyn LedgerStore>,
    mut caller: Caller<'_, ()>,
    key_ptr: i32,
    key_len: i32,
    value_ptr: i32,
    value_len: i32,
) {
    let memory: Memory = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => panic!("`memory` export not found"),
    };
    let key = {
        let mem_data = memory.data(&caller);
        let key = std::str::from_utf8(
            mem_data
                .get(key_ptr as u32 as usize..)
                .and_then(|arr| arr.get(..key_len as u32 as usize))
                .unwrap(),
        )
        .unwrap();
        key.to_string()
    };

    let value = {
        let mem_data = memory.data(&caller);

        mem_data
            .get(value_ptr as u32 as usize..)
            .and_then(|arr| arr.get(..value_len as u32 as usize))
            .unwrap()
            .to_vec()
    };
    ledger_store.put(&key, &value).unwrap();
    println!("Put key: {key} value: {value:?}");
}
