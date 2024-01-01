#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WasmsgParam {
    pub input: Vec<u8>,
    pub signers: Vec<String>,
}

pub mod error;
