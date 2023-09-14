use alloc::vec::Vec;
use purewasm_crypto::id::DigestId;
use serde::{Deserialize, Serialize};
use purewasm_core::PureResult;

// Generic event, it contains wasm id and event kind
#[derive(Serialize, Deserialize, Debug)]
pub struct GenericEvent<EventKind> {
    pub wasm_id: DigestId,
    pub event: EventKind,
}

//
#[derive(Serialize, Deserialize, Debug)]
pub struct WrappedResult {
    pub wasm_id: DigestId,
    pub result: Vec<u8>,
}

// Store of event
#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedEvent {
    pub wasm_id: DigestId,   // wasm identifier
    pub event_bytes: Vec<u8>,      // GenericEvent bytes
    pub result_id: DigestId, // To verify result
}

pub type EventResult = PureResult<WrappedResult>;
