use purewasm_cbor::CborCodec;
use purewasm::{Codec, PureResult};
use purewasm::wasmtime::PureModule;
use purewasm_simple_core::{CustomResult, Input};

fn main() {
    let wasm_file = "../target/wasm32-unknown-unknown/release/purewasm_simple_module.wasm";
    let input = CborCodec.to_bytes(&Input { code: 6 }).unwrap();
    let mut module = PureModule::from_file(wasm_file);
    let result = module.call_fn("handle_example", &input);
    let result: PureResult<CustomResult> = CborCodec::from_bytes(&result).unwrap();
    println!("Result: {:?}", result);
}
