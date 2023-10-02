# rust-purewasm

> rust-purewasm is a purewasm library for rust. 


## Installation

```
cargo install rust-purewasm
```

## Usage

### Module 

``` purewasm = { version = "0.1.0-alpha", features = ["bindgen-json"] } ```

```rust
#![no_main]
#![cfg_attr(not(test), no_std)]
extern crate alloc;
use purewasm::bindgen::prelude::*;

use serde::{Serialize, Deserialize};
use alloc::{format, string::String};

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResult {
    pub msg: String,
}

purewasm_setup!();

#[purewasm_bindgen]
pub fn handle_example(input: Input) -> PureResult<CustomResult> {
    Ok(CustomResult {
        msg: format!("The input code is {}", input.code),
    })
}
```

### Runtime(wasmtime)

``` purewasm = { version = "0.1.0-alpha", features = ["wasmtime"] } ```

```rust
use purewasm::wasmtime::PureModule;

fn main() {
    let wasm_file = "../target/wasm32-unknown-unknown/release/purewasm_json_module.wasm";
    let input = serde_json::to_vec(&serde_json::json!({"code": 6})).unwrap();
    let mut module = PureModule::from_file(wasm_file);
    let result = module.call_fn("handle_example", &input);
    let result: serde_json::Value = serde_json::from_slice(&result).unwrap();
    println!("Result: {:?}", result);
}

```

## License

APACHE 2.0