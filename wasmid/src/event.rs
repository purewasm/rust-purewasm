use alloc::vec::Vec;
use purewasm_crypto::id::DigestId;
use serde::{Deserialize, Serialize};

// Generic event, it contains wasm id and event kind
#[derive(Serialize, Deserialize, Debug)]
pub struct GenericEvent<EventKind> {
    pub context: DigestId,
    pub event: EventKind,
}

//
#[derive(Serialize, Deserialize, Debug)]
pub struct WrappedResult {
    pub context: DigestId,
    pub result: Vec<u8>,
}

// Store of event
#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedEvent {
    pub context: DigestId,   // wasm identifier
    pub event_bytes: Vec<u8>,      // GenericEvent bytes
    pub result_id: DigestId, // To verify result
}
