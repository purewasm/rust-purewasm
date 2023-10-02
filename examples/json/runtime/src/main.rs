use purewasm::wasmtime::PureModule;

fn main() {
    let wasm_file = "../target/wasm32-unknown-unknown/release/purewasm_json_module.wasm";
    let input = serde_json::to_vec(&serde_json::json!({"code": 6})).unwrap();
    let mut module = PureModule::from_file(wasm_file);
    let result = module.call_fn("handle_example", &input);
    let result: serde_json::Value = serde_json::from_slice(&result).unwrap();
    println!("Result: {:?}", result);
}
