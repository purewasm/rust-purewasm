#![cfg_attr(not(test), no_std)]

#[cfg(any(feature = "bindgen", feature = "bindgen-json"))]
pub use purewasm_bindgen as bindgen;

#[cfg(feature = "bindgen")]
pub use purewasm_cbor::CborCodec as DefaultCodec;
#[cfg(feature = "bindgen-json")]
pub use purewasm_json::JsonCodec as DefaultCodec;

#[cfg(feature = "wasmtime")]
pub use purewasm_wasmtime as wasmtime;

