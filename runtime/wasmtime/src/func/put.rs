use std::sync::Arc;

use wasmtime::Caller;

use crate::KvStore;

pub fn put_fn(
    kv_store: Arc<dyn KvStore>,
    mut caller: Caller<'_, ()>,
    ptr: i32,
    len: i32,
) -> (i32, i32) {
    (0, 0)
}
