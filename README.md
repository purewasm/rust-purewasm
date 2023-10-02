# rust-purewasm

> rust-purewasm is a purewasm library for rust. 

The rust-purewasm library is a Rust library designed to build pure functions using WebAssembly (Wasm). WebAssembly is a portable binary intermediate language that enables code execution in a virtual machine. It provides a secure sandboxed environment where code can run, with a limited instruction set that focuses on pure functions and numerical operations on its own memory. While WebAssembly is commonly used in web technology, it is also suitable for developing smart contracts in various contexts, not just limited to blockchain applications.

However, WebAssembly has a limitation when it comes to supporting complex data types. Despite this limitation, rust-purewasm aims to work around it by making assumptions about memory and providing encoding-decoding capabilities for JSON and CBOR codecs.

#### Runtime

The runtime component of rust-purewasm handles the execution of WebAssembly modules. It follows these steps:

- Encodes the input data into bytes using the specified codec (such as JSON or CBOR).
- Allocates memory to store the input bytes.
- Passes the pointer and length of the input to the WebAssembly module.

#### Wasm Module

The wasm module component of rust-purewasm is responsible for processing the input data, preparing the output data, and returning it to the runtime. It performs the following tasks:

- Retrieves the input bytes using the provided pointer and length.
- Decodes the input data from the specified codec (such as JSON or CBOR) into the appropriate data types.
- Processes the input and prepares the output data.
- Encodes the output data into bytes using the specified codec.
- Returns the pointer and length of the output data to the runtime.

Here's an example of how to define a function in Rust that can be used as a WebAssembly module with JSON and CBOR codecs:
```rust
fn example(ptr: i32, len: i32) -> (i32, i32) {
    // ...
}
```


## Installation

```
cargo install rust-purewasm
```

## Usage

### Module 

``` purewasm = { version = "0.1.0", features = ["bindgen-json"] } ```

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

``` purewasm = { version = "0.1.0", features = ["wasmtime"] } ```

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