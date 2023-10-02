use purewasm_cbor::CborCodec;
use purewasm_core::{Codec, PureResult};
use purewasm_simple_shared::{CustomResult, Input};
use purewasm_wasmtime::PureModule;

fn main() {
    let wasm_file = "../target/wasm32-unknown-unknown/release/purewasm_simple_module.wasm";
    let input = purewasm_simple_shared::Input { code: 6 };
    let input_bytes: Vec<u8> = CborCodec.to_bytes(&input).unwrap();
    let mut module = PureModule::from_file(wasm_file);
    let result = module.call_fn("example", &input_bytes);
    let result: PureResult<CustomResult> = CborCodec::from_bytes(&result).unwrap();
    println!("Result: {:?}", result);
}
