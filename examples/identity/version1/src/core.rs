#![cfg_attr(not(test), no_std)]

use serde::{Serialize, Deserialize};

const PUBKEY_SIZE: u8 = 32;
const SIG_SIZE: u16 = 1340;

type IdPublicKey = [u8; PUBKEY_SIZE];

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdEventBody {
    min_signer: u8, // m of n
    total_signer: u8, // total number of signers
    signers: Vec<ContentId>, // New signer ids 
    state: State // Current state of id
}

IdSignature {
    signer_pk: IdPublicKey,
    next_signer_id: ContentId,
    sig_bytes: [u8; SIG_SIZE],
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdEventPayload {
    previous: ContentId,
    body: IdEventBody,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum IdInput {
    Inception(IdEvent),
    Mutation {
        payload: IdEventPayload,
        signatures: [IdSignature; MAX_SIG_SIZE]
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum WrappedResult {
    Version1(IdResult),
}

pub fn handle(input: IdInput) -> Result<WrappedResult>{
    // logic
}