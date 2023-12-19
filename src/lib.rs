#![cfg_attr(not(test), no_std)]

#[cfg(any(feature = "bindgen"))]
pub use purewasm_bindgen as bindgen;

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen-cbor")] {
        pub type CodecImpl = purewasm_cbor::CborCodec;
    }else if #[cfg(feature = "bindgen-json")]{
        pub type CodecImpl = purewasm_json::JsonCodec;
    }else{
        compile_error!("Please enable one of the following features: bindgen-cbor, bindgen-json");
    }
}

#[cfg(feature = "wasmtime")]
pub use purewasm_wasmtime as wasmtime;

