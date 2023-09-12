use crate::{id::DigestId, PureResult};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericEvent<EventKind> {
    wam_id: DigestId,
    event: EventKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WrappedResult {
    wam_id: DigestId,
    result: Vec<u8>,
}

pub type EventResult = PureResult<WrappedResult>;
