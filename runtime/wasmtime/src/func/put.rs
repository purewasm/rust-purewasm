use std::sync::Arc;

use wasmtime::Caller;

use crate::LedgerStore;

pub fn put_fn(
    ledger_store: Arc<dyn LedgerStore>,
    mut caller: Caller<'_, ()>,
    ptr: i32,
    len: i32,
) -> (i32, i32) {
    (0, 0)
}
