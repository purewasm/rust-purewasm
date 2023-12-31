#![no_main]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

use wasmledger_bindgen::prelude::*;

use alloc::{format, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub name: String,
    pub birthday: u32,
    pub events: Vec<UserEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub name: String,
    pub birthday: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub name: String,
    pub birthday: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserInput {
    pub username: String,
    pub deleted_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeleteddEvent {
    pub at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserEvent {
    Created(UserCreatedEvent),
    Deleted(UserDeleteddEvent),
}

#[wasmledger_bindgen]
pub fn create<H: Host>(host: H, input: CreateUserInput) -> Result<(), WasmError> {
    let key = format!("/users/{}", input.username);
    let exist: Option<User> = host.get(&key)?;
    if exist.is_some() {
        return Err(WasmError::code("USER_EXIST"));
    }
    let event = UserEvent::Created(UserCreatedEvent {
        name: input.name.clone(),
        birthday: input.birthday,
    });
    let mut events: Vec<UserEvent> = Vec::new();
    events.push(event);
    let user = User {
        username: input.username,
        name: input.name,
        birthday: input.birthday,
        events: events,
    };
    host.put(&key, &user)?;
    Ok(())
}


//#[wasmledger_bindgen]
pub fn delete<H: Host>(host: H, input: DeleteUserInput) -> Result<(), WasmError> {
    let key = format!("/users/{}", input.username);
    let exist: Option<User> = host.get(&key)?;
    if let Some(mut user) = exist {
        // check if user already deleted
        if user.events.iter().any(|e| match e {
            UserEvent::Deleted { .. } => true,
            _ => false,
        }) {
            return Err(WasmError::code("already deleted"));
        }
        user.events.push(UserEvent::Deleted(UserDeleteddEvent {
            at: input.deleted_at,
        }));
        host.put(&key, &user)?;
        return Ok(());
    }
    Err(WasmError::code("NOT_FOUND"))
}
