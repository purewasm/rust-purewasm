use alloc::{collections::BTreeMap, vec::Vec};
use purewasm_codec::cbor::CborCodec;
use purewasm_core::{Codec, DigestId, PureError};
use purewasm_crypto::verification::{IdPublicKeyKind, IdSignatureKind};
use serde::{Deserialize, Serialize};

// Generic event, it contains wasm id and event kind
#[derive(Serialize, Deserialize, Debug)]
pub struct IdEvent {
    pub context: DigestId,
    pub payload: IdEventKind,
}

//
#[derive(Serialize, Deserialize, Debug)]
pub struct WrappedResult {
    pub context: DigestId,
    pub result: Vec<u8>,
}

// Store of event
#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedEvent {
    pub context: DigestId,   // wasm identifier
    pub payload: Vec<u8>,    // GenericEvent bytes
    pub result_id: DigestId, // To verify result
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdInception {
    pub min_signer: u8,         // m of n
    pub total_signer: u8,       // total number of signers
    pub signers: Vec<DigestId>, // New signer ids
    pub sdt_state: DigestId,    // Current state of id
}

impl IdInception {
    pub fn get_id(&self) -> Result<DigestId, PureError> {
        DigestId::new_sha256(&CborCodec.to_bytes(&self)?)
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

impl IdMutation {
    pub fn get_id(&self) -> Result<DigestId, PureError> {
        DigestId::new_sha256(&CborCodec.to_bytes(&self)?)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum IdEventKind {
    Inception(IdInception),
    Mutation(IdMutation),
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
