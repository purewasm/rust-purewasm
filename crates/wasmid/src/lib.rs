#![no_main]
#![cfg_attr(not(test), no_std)]
mod model;
mod handler;
mod wots;
use purewasm_bindgen::purewasm_bindgen;
use purewasm_core::use_purewasm;
use purewasm_core::event::{EventResult, GenericEvent};
use model::IdEventKind;

use_purewasm!();


#[purewasm_bindgen]
pub fn handle_event(input: GenericEvent<IdEventKind>) -> EventResult {
    handler::handle(input)
}