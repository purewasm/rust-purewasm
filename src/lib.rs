#![cfg_attr(not(test), no_std)]

pub use purewasm_core::*;

#[cfg(any(feature = "bindgen", feature = "bindgen-json"))]
pub use purewasm_bindgen as bindgen;

#[cfg(any(feature = "wasmtime", feature = "wasmtime-json"))]
pub use purewasm_wasmtime as wasmtime;
