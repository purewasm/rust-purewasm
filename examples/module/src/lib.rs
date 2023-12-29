#![no_main]
#![cfg_attr(not(test), no_std)]
extern crate alloc;
use purewasm_bindgen::prelude::*;

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
    pub deleted_at: i64
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub name: String,
    pub events: Vec<UserEvent>,
}

#[purewasm_bindgen]
pub fn create(input: CreateUserInput) -> Result<(), WasmError> {
    let key = format!("/users/{}", input.username);
    let exist: Option<User> = get!(key)?;
    if exist.is_some() {
        return Err(WasmError::Other("exist".to_string()));
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
    put!(&key, user);
    Ok(())
}

#[purewasm_bindgen]
pub fn delete(input: DeleteUserInput) -> Result<(), WasmError> {
    let key = format!("/users/{}", input.username);
    let user: Option<User> = get!(key)?;
    if let Some(mut user) = user {
        // check if user already deleted
        if user.events.iter().any(|e| match e {
            UserEvent::Deleted { .. } => true,
            _ => false,
        }){
            return Err(WasmError::Other("already deleted".to_string()));
        }
        user.events.push(UserEvent::Deleted(UserDeleteddEvent { at: input.deleted_at }));
        put!(&key, user);
        return Ok(());
    }
    Err(WasmError::NotFound)
}
