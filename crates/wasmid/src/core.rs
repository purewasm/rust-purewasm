use alloc::{collections::BTreeMap, vec::Vec};
use purewasm_core::{
    codec::{cbor::CborCodec, Codec},
    error::PureError,
    event::{EventResult, GenericEvent, WrappedResult},
    id::DigestId,
    serde::{Deserialize, Serialize},
    serde_utils::serde_bytes_array,
};

use crate::impls;

const PUBKEY_SIZE: usize = 32;
const SIG_SIZE: usize = 1340;
pub type IdPublicKey = [u8; PUBKEY_SIZE];

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub struct IdInception {
    min_signer: u8,         // m of n
    total_signer: u8,       // total number of signers
    signers: Vec<DigestId>, // New signer ids
    sdt_state: DigestId,    // Current state of id
}

impl IdInception {
    pub fn get_id(&self) -> DigestId {
        DigestId::Sha256([0; 32])
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub struct IdMutation {
    pub previous: WrappedResult,                   // wam_id, bytes
    pub min_signer: Option<u8>,                    // m of n
    pub total_signer: Option<u8>,                  // total number of signers
    pub new_signers: BTreeMap<DigestId, DigestId>, // New signer ids
    pub sdt_state: Option<DigestId>,               // Current state of id
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub struct IdSignature {
    pub signer_id: DigestId,
    pub signer_pk: IdPublicKey,
    pub next_signer_id: DigestId,
    #[serde(with = "serde_bytes_array")]
    pub sig_bytes: [u8; SIG_SIZE],
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub enum IdEventKind {
    Inception(IdInception),
    Mutation {
        payload: IdMutation,
        signatures: Vec<IdSignature>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "purewasm_core::serde")]
pub struct IdEventResult {
    pub  id: DigestId,
    pub event_id: DigestId,
    pub min_signer: u8,
    pub total_signer: u8,
    pub signers: Vec<DigestId>,
    pub sdt_state: DigestId,
}

pub fn handle(input: GenericEvent<IdEventKind>) -> EventResult {
    let result = match input.event {
        IdEventKind::Inception(inception) => {
            let result = IdEventResult {
                id: inception.get_id(),
                event_id: inception.get_id(),
                min_signer: inception.min_signer,
                total_signer: inception.total_signer,
                signers: inception.signers,
                sdt_state: inception.sdt_state,
            };
            result
        }
        IdEventKind::Mutation {
            payload,
            signatures,
        } => {
            if payload.previous.wasm_id == input.wasm_id {
                impls::current::resolve(payload, signatures)?
            }else {
                impls::fake::resolve(payload, signatures)?
                //return Err(PureError::new("NOT_SUPPORTED"));
            }
        }
    };
    
    let bytes = CborCodec.to_bytes(result)?;
    let wrapped = WrappedResult {
       wasm_id: input.wasm_id,
       result: bytes
    };
    Ok(wrapped)
}
