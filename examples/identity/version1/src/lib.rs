#![cfg_attr(not(test), no_std)]

extern crate alloc;

const CID_SIZE: u8 = 32;

type ContentId = [u8; CID_SIZE];

pub struct IdResult {
   id: ContentId, 
   event_id: ContentId,
   min_signer: u8,
   total_signer: u8,
   signers: Vec<ContentId>, 
   state: State
}