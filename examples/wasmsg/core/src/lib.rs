use alloc::{collections::BTreeMap, string::String, vec::Vec};
use purewasm_core::DigestId;
use purewasm_core::PureResult;
use serde::{Deserialize, Serialize};

pub struct PureMessage {
    pub context: DigestId,                 // Wasm identifier
    pub query: BTreeMap<String, DigestId>, // xxx -> id
    pub payload: Vec<u8>,                  // Encoded message payload
    pub event: DigestId,
}

impl PureMessage {
    pub fn get_id(&self) -> Result<DigestId, PureError> {
        DigestId::new_sha256(&CborCodec.to_bytes(&self)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInput {
    pub context: Vec<u8>,                 // Wasm bytes
    pub query: BTreeMap<String, Vec<u8>>, // xxx -> encoded query result
    pub message: PureMessage,             // Persisted message
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
    pub context: DigestId,
    pub payload: Vec<u8>, // Encoded event
}

pub type PureMsgResult = PureResult<PureEvent>;

pub fn handle(input: PureMessageInput) -> PureMsgResult {
    // each hash(input.query) == input.message.query_ids
    let result = PureEvent {
        context: input.message.context,
        event: Vec::new(),
    };
    // hash(result) == input.message.query.event_id
    Ok(result)
}