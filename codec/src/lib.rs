#![cfg_attr(not(test), no_std)]
extern crate alloc;
use alloc::vec::Vec;
use purewasm_core::PureError;
use serde::{de::DeserializeOwned, Serialize};
pub mod cbor;
pub mod json;

