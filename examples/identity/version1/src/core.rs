#![cfg_attr(not(test), no_std)]

use serde::{Serialize, Deserialize};
type IdPublicKey = [u8; PUBKEY_SIZE];

const PUBKEY_SIZE: u8 = 32;
const SIG_SIZE: u16 = 1340;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdInception {
    min_signer: u8, // m of n
    total_signer: u8, // total number of signers
    signers: Vec<DigestId>, // New signer ids 
    sdt_state: DigestId // Current state of id
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdMutation {
    previous: WrappedResult, // wam_id, bytes
    min_signer: Option<u8>, // m of n
    total_signer: Option<u8>, // total number of signers
    new_signers: Vec<DigestId>, // New signer ids 
    sdt_state: Option<DigestId> // Current state of id
}

IdSignature {
    signer_id: DigestId,
    signer_pk: IdPublicKey,
    next_signer_id: DigestId,
    sig_bytes: [u8; SIG_SIZE],
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum IdEventKind {
    Inception(IdInception),
    Mutation {
        payload: IdMutation,
        signatures: Vec<IdSignature>
    }
}

pub struct IdEventResult {
    id: DigestId, 
    event_id: DigestId,
    min_signer: u8,
    total_signer: u8,
    signers: BTreeMap<DigestId, DigestId>, 
    sdt_state: DigestId
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
                sdt_state: inception.sdt_state
             };
             result 
        },
        IdEventKind::Mutation{
            payload, 
            signatures
        } => {
           if signatures.len() < payload.previous.min_signer {
              return Err(PureError::new("MIN_SIGNATURE"))
           }
           let mut new_signers: Vec<DigestId> = Vec::new();
           let mut signers = payload.previous.signers.clone();
           for sig in signatures {
              sig.signer_id.ensure(sig.signer_pk);
              if !signers.contain(sig.signer_id) {
                 return Err(PureError::new("UNKNOWN_SIGNER"))
              }else{
                 sig.verify(payload);
                 new_signers.push(sig.next_signer_id);
                 signers.remove(sig.signer_id);
              }
           }

           let result = IdEventResult {
              id: previous.id, 
              event_id: input.event.get_id(),
              min_signer: payload.min_signer ?? payload.previous.min_signer,
              total_signer: payload.total_signer ?? payload.previous.total_signer,
              signers: new_signers, 
              sdt_state: payload.sdt_state ?? payload.previous.sdt_state
           };
           result 
        }
    }
    let bytes = encode(result);
    let wrapped = WrappedResult {
       wasm_id: input.wasm_id,
       result: bytes
    };
    Ok(wrapped)
}