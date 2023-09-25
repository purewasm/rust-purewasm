pub struct NetworkMessage {
    pub channel: DigestId,                 // 
    pub previous: DigestId,                // Previous message id
    pub payload: PureMessage,              //
    pub timestamp: Vec<u8>                 // Org timestamp
}

impl NetworkMessage {
    pub fn get_id(&self) -> Result<DigestId, PureError> {
        DigestId::new_sha256(&CborCodec.to_bytes(&self)?)
    }
}

pub fn handle(msg: NetworkMessage) {
    // check channel
    // check previous is last event
    // check timestamp is valid
    // check context is valid
    // get queries from channel db
    // create message input
    // execute wasm with input 
    // check result event and update channel db 
}