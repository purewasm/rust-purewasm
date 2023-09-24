use alloc::{collections::BTreeMap, string::String, vec::Vec};
use purewasm_core::DigestId;
use purewasm_core::PureResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PureMessage {
    pub previous: DigestId,                // Previous message id
    pub context: DigestId,                 // wasm identifier
    pub query: BTreeMap<String, DigestId>, // db/xxx -> id
    pub command: Vec<u8>,                  // Encoded message payload
    pub event: DigestId,                   // id of events, should be same with wasm execution
}

impl PureMessage {
    pub fn get_id(&self) -> Result<DigestId, PureError> {
        DigestId::new_sha256(&CborCodec.to_bytes(&self)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PureMessageInput {
    pub context: Vec<u8>,                 // wasm raw bytes
    pub query: BTreeMap<String, Vec<u8>>, // db:xxx -> encoded query result
    pub message: PureMessage,             // Persisted message
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PureEvent {
    pub context: DigestId,
    pub event: Vec<u8>, // Encoded event
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