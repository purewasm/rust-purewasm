use serde::{Serialize, Deserialize};
use alloc::vec::Vec;
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum DigestId {
    Sha256([u8; 32]),
}

impl DigestId {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
