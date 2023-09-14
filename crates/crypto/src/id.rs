use serde::{Serialize, Deserialize};
use alloc::vec::Vec;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum DigestId {
    Sha256([u8; 32]),
}

impl DigestId {
    pub fn to_bytes(&self) -> Vec<u8> {
        use sha2::digest::Digest;
        //use sha2::digest::Update;
        let hasher: [u8;32] =sha2::Sha256::digest(&[0u8;32]).try_into().unwrap();
        todo!()
    }
}
