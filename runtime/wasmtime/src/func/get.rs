use std::sync::Arc;

use wasmtime::{Caller, Memory, Extern, TypedFunc};

use crate::LedgerStore;

pub fn get_fn(
    kv_store: Arc<dyn LedgerStore>,
    mut caller: Caller<'_, ()>,
    ptr: i32,
    len: i32,
) -> (i32, i32) {
    let memory: Memory = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => panic!("`memory` export not found"),
    };
    let alloc = match caller.get_export("alloc") {
        Some(Extern::Func(func)) => func,
        _ => panic!("`alloc` export not found"),
    };
    let de_alloc = match caller.get_export("de_alloc") {
        Some(Extern::Func(func)) => func,
        _ => panic!("`de_alloc` export not found"),
    };
    let key = {
        let mem_data = memory.data(&caller);
        let key = std::str::from_utf8(
            mem_data
                .get(ptr as u32 as usize..)
                .and_then(|arr| arr.get(..len as u32 as usize))
                .unwrap(),
        )
        .unwrap();
        key.to_string()
    };

    let alloc_func: TypedFunc<i32, i32> = alloc.typed::<i32, i32>(&caller).unwrap();
    let value = kv_store.get(&key);
    let value_len = value.len() as i32;
    let value_ptr = alloc_func.call(&mut caller, value_len).unwrap();
    let mem_data = memory.data_mut(&mut caller);

    mem_data
        .get_mut(value_ptr as usize..)
        .and_then(|s| s.get_mut(..value_len as usize))
        .unwrap()
        .copy_from_slice(&value);
    (value_ptr, value_len)
}
