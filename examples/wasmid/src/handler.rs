use crate::{
    contants::error_codes,
    model::{IdEvent, WrappedResult},
};
use alloc::vec::Vec;
use purewasm_codec::{cbor::CborCodec, Codec};
use purewasm_crypto::id::DigestId;
use purewasm_model::{PureError, PureResult};

use crate::model::{IdEventKind, IdEventResult};

pub fn handle(input: IdEvent) -> PureResult<WrappedResult> {
    let result = match input.payload {
        IdEventKind::Inception(inception) => {
            let result = IdEventResult {
                id: inception.get_id()?,
                event_id: inception.get_id()?,
                min_signer: inception.min_signer,
                total_signer: inception.total_signer,
                signers: inception.signers,
                sdt_state: inception.sdt_state,
            };
            result
        }
        IdEventKind::Mutation(mutation) => {
            let event_id = mutation.get_id()?;
            // Check previous context is same with current
            if mutation.payload.previous.context == input.context {
                let previous: IdEventResult =
                    CborCodec::from_bytes(&mutation.payload.previous.result)?;
                if mutation.signatures.len() < previous.min_signer as usize {
                    return Err(PureError::new(error_codes::MIN_SIGNATURE_NOT_MATCH));
                }
                let mut next_signers: Vec<DigestId> = Vec::new();
                let mut signers: Vec<DigestId> = previous.signers.clone();

                for sig in mutation.signatures {
                    sig.signer_id.ensure(&sig.signer_pk.to_bytes())?;
                    if !signers.contains(&sig.signer_id) {
                        return Err(PureError::new(error_codes::UNKNOWN_SIGNER));
                    }
                    if sig.next_signer_id == sig.signer_id {
                        return Err(PureError::new(error_codes::SIGNER_SHOULD_BE_DIFFERENT));
                    }
                    let payload_bytes = CborCodec.to_bytes(&mutation.payload)?;
                    sig.signer_pk
                        .verify(&payload_bytes, sig.sig_bytes)
                        .map_err(|err| PureError::new(err))?;

                    next_signers.push(sig.next_signer_id);
                    signers.retain(|value| *value != sig.signer_id);
                }
                for (ex, next) in mutation.payload.new_signers {
                    if !signers.contains(&ex) {
                        return Err(PureError::new(error_codes::UNKNOWN_SIGNER));
                    }
                    if ex == next {
                        return Err(PureError::new(error_codes::SIGNER_SHOULD_BE_DIFFERENT));
                    }
                    next_signers.push(next);
                    signers.retain(|value| *value != ex);
                }
                let result = IdEventResult {
                    id: previous.id,
                    event_id: event_id,
                    signers: next_signers,
                    min_signer: if let Some(ms) = mutation.payload.min_signer {
                        ms
                    } else{
                        previous.min_signer
                    },
                    total_signer: if let Some(ts) = mutation.payload.total_signer {
                        ts
                    } else{
                        previous.total_signer
                    },
                    sdt_state: if let Some(sdt) = mutation.payload.sdt_state {
                        sdt
                    } else{
                        previous.sdt_state
                    },
                }; 
                result
            } else {
                return Err(PureError::new(error_codes::UNKNOWN_CONTEXT));
            }
        }
    };

    let bytes = CborCodec.to_bytes(&result)?;
    let wrapped = WrappedResult {
        context: input.context,
        result: bytes,
    };
    Ok(wrapped)
}
