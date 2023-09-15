use crate::{event::{ GenericEvent, WrappedResult}, contants::error_codes};
use purewasm_core::{codec::Codec, PureResult};
use  purewasm_error::PureError;

use crate::{
    codec::CborCodec,
    model::{IdEventKind, IdEventResult},
};

pub fn handle(input: GenericEvent<IdEventKind>) -> PureResult<WrappedResult>  {
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
        IdEventKind::Mutation(mutation) => {
            // Check previous context is same with current
            if mutation.payload.previous.context == input.context {
                current::handle(mutation.payload, mutation.signatures)?
            } else {
                return Err(PureError::new(error_codes::UNKNOWN_CONTEXT));
            }
        }
    };

    let bytes = CborCodec.to_bytes(result)?;
    let wrapped = WrappedResult {
        context: input.context,
        result: bytes,
    };
    Ok(wrapped)
}

pub mod current {
    use alloc::vec::Vec;

    use purewasm_core::codec::Codec;
    use purewasm_error::PureError;
    use purewasm_crypto::id::DigestId;

    use crate::{
        codec::CborCodec,
        contants::error_codes,
        model::{IdEventResult, IdMutationPayload, IdSignature},
    };

    pub fn handle(
        payload: IdMutationPayload,
        signatures: Vec<IdSignature>,
    ) -> Result<IdEventResult, PureError> {
        let previous: IdEventResult = CborCodec::from_bytes(&payload.previous.result)?;
        if signatures.len() < previous.min_signer as usize {
            return Err(PureError::new(error_codes::MIN_SIGNATURE_NOT_MATCH));
        }
        let mut new_signers: Vec<DigestId> = Vec::new();
        for sig in signatures {
            sig.signer_id
                .ensure(&sig.signer_pk.to_bytes())
                .map_err(|err| PureError::new(err))?;
            if !previous.signers.contains(&sig.signer_id) {
                return Err(PureError::new("UNKNOWN_SIGNER"));
            } else {
                let payload_bytes = CborCodec.to_bytes(&payload)?;
                sig.signer_pk
                    .verify(&payload_bytes, sig.sig_bytes)
                    .map_err(|err| PureError::new(err))?;
                if sig.next_signer_id == sig.signer_id {
                    return Err(PureError::new("NEW_SIGNER_EX_SIGNER"));
                }
                new_signers.push(sig.next_signer_id);
            }
        }
        for sig in payload.new_signers {}
        todo!()
    }
}
