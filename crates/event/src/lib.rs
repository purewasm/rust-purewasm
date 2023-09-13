#![cfg_attr(not(test), no_std)]

extern crate alloc;
use purewasm_core::{id::DigestId, PureResult};
use alloc::vec::Vec;
use purewasm_core::serde::{Deserialize, Serialize};

// Generic event, it contains wasm id and event kind
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub struct GenericEvent<EventKind> {
    pub wasm_id: DigestId,
    pub event: EventKind,
}

// 
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub struct WrappedResult {
    pub wasm_id: DigestId,
    pub result: Vec<u8>,
}

// Store of event
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub struct WrappedEvent {
    pub wasm_id: DigestId, // wasm identifier
    pub event: Vec<u8>, // GenericEvent bytes
    pub result_id: DigestId, // To verify result
}

pub type EventResult = PureResult<WrappedResult>;
