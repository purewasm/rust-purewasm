use alloc::{collections::BTreeMap, vec::Vec};
use purewasm_crypto::{id::DigestId, verification::{IdPublicKeyKind, IdSignatureKind}};
use serde::{Deserialize, Serialize};
use crate::event::WrappedResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct IdInception {
    pub min_signer: u8,         // m of n
    pub total_signer: u8,       // total number of signers
    pub signers: Vec<DigestId>, // New signer ids
    pub sdt_state: DigestId,    // Current state of id
}

impl IdInception {
    pub fn get_id(&self) -> DigestId {
        DigestId::Sha256([0; 32])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdMutationPayload {
    pub previous: WrappedResult,                   // wam_id, bytes
    pub min_signer: Option<u8>,                    // m of n
    pub total_signer: Option<u8>,                  // total number of signers
    pub new_signers: BTreeMap<DigestId, DigestId>, // New signer ids
    pub sdt_state: Option<DigestId>,               // Current state of id
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdSignature {
    pub signer_id: DigestId,
    pub signer_pk: IdPublicKeyKind,
    pub next_signer_id: DigestId,
    pub sig_bytes: IdSignatureKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdMutation {
    pub payload: IdMutationPayload,
    pub signatures: Vec<IdSignature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IdEventKind {
    Inception(IdInception),
    Mutation(IdMutation)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdEventResult {
    pub id: DigestId,
    pub event_id: DigestId,
    pub min_signer: u8,
    pub total_signer: u8,
    pub signers: Vec<DigestId>,
    pub sdt_state: DigestId,
}
