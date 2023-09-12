pub mod current {
    use alloc::vec::Vec;

    use purewasm_core::{
        codec::{cbor::CborCodec, Codec},
        error::PureError,
        id::DigestId,
    };

    use crate::core::{IdEventResult, IdMutation, IdSignature};

    pub fn resolve(
        payload: IdMutation,
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

pub mod fake {
    use alloc::vec::Vec;

    use purewasm_core::{
        codec::{cbor::CborCodec, Codec},
        error::PureError,
        id::DigestId,
    };

    use crate::core::{IdEventResult, IdMutation, IdSignature};

    pub fn resolve(
        payload: IdMutation,
        signatures: Vec<IdSignature>,
    ) -> Result<IdEventResult, PureError> {
        let previous: IdEventResult = CborCodec::from_bytes(&payload.previous.result)?;
        if signatures.len() < previous.min_signer as usize {
            return Err(PureError::new("MIN_SIGNATURE"));
        }
        let mut new_signers: Vec<DigestId> = Vec::new();
        todo!()
    }
}

/*let result = match input.event {
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
        if(payload.previous.wasm_id != input.wasm_id){
            return Err(PureError::new("MIN_SIGNATURE"));
        }



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
       for sig in payload.new_signers {

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
}*/
