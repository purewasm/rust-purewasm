#![no_main]
#![cfg_attr(not(test), no_std)]
mod model;
mod handler;
mod codec;
use purewasm_bindgen::purewasm_bindgen;
use purewasm_alloc::use_purewasm;
use purewasm_event::{EventResult, GenericEvent};
use model::IdEventKind;

use_purewasm!();
use crate::codec::CborCodec as DefaultCodec;

#[purewasm_bindgen]
pub fn handle_event(input: GenericEvent<IdEventKind>) -> EventResult {
    handler::handle(input)
}