# rust-purewasm

> rust-purewasm is a purewasm library for rust. 


## Installation

```
cargo install rust-purewasm
```

## Usage

### Module 

CBOR

``` purewasm = { version = "0.1.0-alpha", features = ["bindgen"] } ```

----------
JSON

``` purewasm = { version = "0.1.0-alpha", features = ["bindgen-json"] } ```

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResult {
    pub msg: String,
}

#![no_main]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::{format, string::String};
use purewasm::bindgen::prelude::*;

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

use purewasm_cbor::CborCodec;
use purewasm_core::{Codec, PureResult};
use purewasm_wasmtime::PureModule;

fn main() {
    let wasm_file = "../target/wasm32-unknown-unknown/release/purewasm_simple_module.wasm";
    let input = CborCodec.to_bytes(&Input { code: 6 }).unwrap();
    let mut module = PureModule::from_file(wasm_file);
    let result = module.call_fn("handle_example", &input);
    let result: PureResult<CustomResult> = CborCodec::from_bytes(&result).unwrap();
    println!("Result: {:?}", result);
}
```

## License

APACHE 2.0