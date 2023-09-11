use crate::{error::PureError, DigestId};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize };

pub type PureResult<T> = Result<T, PureError>;

#[derive(Serialize, Deserialize, Debug)]
struct WrappedResult {
    wam_id: DigestId,
    result: Vec<u8>,
}
