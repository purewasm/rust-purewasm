use alloc::{string::String, vec::Vec};
use serde::{Serialize, Deserialize};
use wasmledger_bindgen::prelude::*;

/// Wasm message
/// 
/// A message in the wasmledger protocol
/// 
/// The message contains the payload and the proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wasmsg {
    /// The message payload
    pub payload: Vec<u8>,
    /// The proofs
    pub proofs: Vec<u8>,
}

/// Wasm message proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmsgProof {
    /// The signer
    pub signer: String,
    /// The signature
    pub sig: Vec<u8>,
}

/// Block verification
/// 
/// Verify a block header
/// 
/// # Parameters
/// 
/// * ptr: pointer to the block header
/// * len: length of the block header
/// 
/// # Returns
/// 
/// - 1 if verification is successful
/// - 0 otherwise
/// 
#[no_mangle]
pub unsafe extern "C" fn verify_block(ptr: *mut u8, len: i32) -> i32 {
    let memory = WasmMemory {
        codec: CodecImpl {},
    };
    let msg: Wasmsg = memory.from_memory(ptr, len).unwrap();
    // payload
    // proof
    let msg = core::slice::from_raw_parts(ptr, len as usize);
    let payload_len = &msg[..4];
    let payload_len = u32::from_le_bytes(payload_len.try_into().unwrap());
    let payload = msg[4..payload_len as usize + 4].to_vec();
    1
}

// Verify a block header
// If verification is successful, return (ptr, len) otherwise (0, 0)
// ptr: pointer to the signer list and others
// len: length of the signer list and others
#[no_mangle]
pub unsafe extern "C" fn verify_msg(ptr: *mut u8, len: i32) -> (i32, i32) {
    let memory = WasmMemory {
        codec: CodecImpl {},
    };
    let msg: Wasmsg = memory.from_memory(ptr, len).unwrap();
    // payload
    // proof
    let msg = core::slice::from_raw_parts(ptr, len as usize);
    let payload_len = &msg[..4];
    let payload_len = u32::from_le_bytes(payload_len.try_into().unwrap());
    let payload = msg[4..payload_len as usize + 4].to_vec();
    (0, 0)
}

