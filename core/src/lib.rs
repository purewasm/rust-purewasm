#![cfg_attr(not(test), no_std)]
extern crate alloc;
mod error;
mod codec;

pub use error::PureError;
pub use codec::Codec;
pub type PureResult<T> = Result<T, PureError>;