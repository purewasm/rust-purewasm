#![cfg_attr(not(test), no_std)]

use serde::{Serialize, Deserialize};

const PUBKEY_SIZE: u8 = 32;
const SIG_SIZE: u16 = 1340;
const CID_SIZE: u8 = 32;
const MAX_SIG_SIZE: u8 = 5;

type IdPublicKey = [u8; PUBKEY_SIZE];
type ContentId = [u8; CID_SIZE];
enum State {
   ThirtyTwo([u8; 32]),
   SixtyFour([u8; 64])
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdEvent {
    timestamp: u128, // The time of chain, it is not real timestamp
    min_signer: u8, // m of n
    total_signer: u8, // total number of signers
    signers: [IdPublicKey; MAX_SIG_SIZE], // KeyId bytes 
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
    event: IdEvent,
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
    Version1(Version1Result),
}

pub fn handle(input: IdInput) -> Result<WrappedResult>{
    // logic
}