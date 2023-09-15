#![no_main]
#![cfg_attr(not(test), no_std)]
mod codec;
mod contants;
mod event;
mod handler;
mod model;
use event::{GenericEvent, WrappedResult};
use model::IdEventKind;
use purewasm_core::{purewasm_bindgen, use_purewasm};

use_purewasm!();
use crate::codec::CborCodec as DefaultCodec;

#[purewasm_bindgen]
pub fn handle_event(input: GenericEvent<IdEventKind>) -> PureResult<WrappedResult> {
    handler::handle(input)
}
