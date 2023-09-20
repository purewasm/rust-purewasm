use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct IdProof {
    pub event_proof: MerkleProof,
    pub branch_proof: MerkleProof,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleProof {
    pub index: u32,
    pub length: u32,
    pub proof: Vec<u8>,
}
