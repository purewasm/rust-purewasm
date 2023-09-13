#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub use serde;
pub use sha2;
pub mod id;
pub mod codec;
pub mod error;
pub mod memory;
pub mod serde_utils;

pub type PureResult<T> = Result<T, error::PureError>;
