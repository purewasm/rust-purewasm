#![cfg_attr(not(test), no_std)]

use serde::{Serialize, Deserialize};
type IdPublicKey = [u8; PUBKEY_SIZE];

const PUBKEY_SIZE: u8 = 32;
const SIG_SIZE: u16 = 1340;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdInception {
    min_signer: u8, // m of n
    total_signer: u8, // total number of signers
    signers: Vec<ContentId>, // New signer ids 
    sdt_state: ContentId // Current state of id
}

IdSignature {
    signer_pk: IdPublicKey,
    next_signer_id: ContentId,
    sig_bytes: [u8; SIG_SIZE],
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdMutation {
    previous: IdResult, // Previous bytes
    min_signer: Option<u8>, // m of n
    total_signer: Option<u8>, // total number of signers
    new_signers: Vec<ContentId>, // New signer ids 
    sdt_state: Option<ContentId> // Current state of id
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum IdEventChange {
    Inception(IdInception),
    Mutation {
        payload: IdMutation,
        signatures: Vec<IdSignature>
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdEvent {
    wam_id: ContentId,
    change: IdEventChange
}
