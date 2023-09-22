use alloc::{collections::BTreeMap, string::String, vec::Vec};
use purewasm_core::DigestId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PureMessage {
    pub context: DigestId,                  // wasm identifier
    pub query: BTreeMap<String, DigestId>, // db:xxx -> id
    pub command: Vec<u8>,                      // Intention, raw payload bytes
    pub event: DigestId,                    // id of events, should be same with wasm execution
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PureMessageInput {
    pub message: PureMessage,
    pub query: BTreeMap<String, Vec<u8>>, // db:xxx -> id
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomResult;

pub fn handle(input: PureMessageInput) -> purewasm_core::PureResult<CustomResult> {
    // each hash(input.query) == input.message.query_ids
    let result = CustomResult;
    // hash(result) == input.message.query.event_id
    Ok(result)
}
