use purewasm_core::{
    codec::{cbor::CborCodec, Codec},
    event::{EventResult, GenericEvent, WrappedResult}, error::PureError,
};

use crate::model::{IdEventKind, IdEventResult};

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
        IdEventKind::Mutation(mutation) => {
            if mutation.payload.previous.wasm_id == input.wasm_id {
                current::resolve(mutation.payload, mutation.signatures)?
            } else {
                return Err(PureError::new("NOT_SUPPORTED"));
            }
        }
    };

    let bytes = CborCodec.to_bytes(result)?;
    let wrapped = WrappedResult {
        wasm_id: input.wasm_id,
        result: bytes,
    };
    Ok(wrapped)
}

pub mod current {
    use alloc::vec::Vec;

    use purewasm_core::{
        codec::{cbor::CborCodec, Codec},
        error::PureError,
        id::DigestId,
    };

    use crate::model::{IdEventResult, IdMutationPayload, IdSignature};

    pub fn resolve(
        payload: IdMutationPayload,
        signatures: Vec<IdSignature>,
    ) -> Result<IdEventResult, PureError> {
        let previous: IdEventResult = CborCodec::from_bytes(&payload.previous.result)?;
        if signatures.len() < previous.min_signer as usize {
            return Err(PureError::new("MIN_SIGNATURE"));
        }
        let mut new_signers: Vec<DigestId> = Vec::new();
        /*for sig in signatures {
            sig.signer_id.ensure(sig.signer_pk);
            if !previous.signers.contain(sig.signer_id) {
                return Err(PureError::new("UNKNOWN_SIGNER"));
            } else {
                sig.verify(payload);
                new_signers.push(sig.next_signer_id);
                signers.remove(sig.signer_id);
            }
        }
        for sig in payload.new_signers {}*/
        todo!()
    }
}