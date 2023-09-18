#![no_main]
#![cfg_attr(not(test), no_std)]
mod contants;
mod handler;
mod model;
use model::{IdEvent, WrappedResult};
use purewasm_bindgen::{purewasm_bindgen, purewasm_setup};

purewasm_setup!();
use purewasm_codec::cbor::CborCodec as DefaultCodec;

#[purewasm_bindgen]
pub fn handle_event(input: IdEvent) -> PureResult<WrappedResult> {
    handler::handle(input)
}
