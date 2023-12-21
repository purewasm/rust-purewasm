#![no_main]
#![cfg_attr(not(test), no_std)]
extern crate alloc;
use purewasm::bindgen::prelude::*;

use alloc::{format, string::String};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResult {
    pub msg: String,
}

#[purewasm_bindgen]
pub fn handle_example(input: Input) -> Result<CustomResult, String> {
    put!(
        "profile",
        Profile {
            name: "John".to_string(),
            age: 30
        }
    );
    let profile: Profile = get!("profile")?;
    let msg = format!(
        "The input code is {} and the name is {} and the age is {}",
        input.code, profile.name, profile.age
    );
    Ok(CustomResult { msg: msg })
}
