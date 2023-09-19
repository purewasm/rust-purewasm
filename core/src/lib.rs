#![cfg_attr(not(test), no_std)]
extern crate alloc;
mod error;
mod codec;
mod id;
mod serde_ext;

pub use error::PureError;
pub use codec::Codec;
pub use id::DigestId;
pub use serde_ext::serde_bytes_array;
pub type PureResult<T> = Result<T, PureError>;