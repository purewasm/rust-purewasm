#![cfg_attr(not(test), no_std)]
extern crate alloc;

mod error;
mod handler;
mod model;
mod verification;
mod message;
pub use model::PersistedIdEvent;
pub mod prelude {
    pub use crate::{
        handler::handle,
        model::{IdCommand, WrappedIdEvent},
    };
    pub use purewasm_codec::cbor::CborCodec as DefaultCodec;
}

