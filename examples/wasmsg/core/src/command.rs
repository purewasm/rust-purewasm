pub struct PureCommand {
    pub channel: String,                   // Channel identifier, it is also a db
    pub context: DigestId,                 // Wasm identifier
    pub query: Vec<String>,                // Key value id e.g. xxx -> id
    pub payload: Vec<u8>,                  // Encoded message payload
    pub event: DigestId,                   // Result identifier
}

pub struct PureMessage {
    pub command: PureCommand, // Command payload
    pub previous: DigestId,   // Previous message id
    pub timestamp: Vec<u8>,   // Org timestamp
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PureMessageInput {
    pub query: BTreeMap<String, Vec<u8>>, // xxx -> encoded query result
    pub command: Vec<u8>,                 // Persisted message
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PureMessageEvent {
    pub context: DigestId,
    pub payload: Vec<u8>, // Encoded event
}

pub type PureMsgResult = PureResult<PureEvent>;

// When a client wants to execute a command
pub fn handle_command(cmd: PureCommand) {
    // check channel
    // check context is valid
    // get context, queries and previous from channel db
    // create message input
    // execute wasm with input
    // check result event and update channel db(events, latest message)
    // create message and timestamp
    // propagate message to orgs
}

// When an executed message propagated
pub fn handle_message(msg: PureMessage) {
    // check channel
    // check previous is last event
    // check timestamp is valid
    // check context is valid
    // get queries from channel db
    // create message input
    // execute wasm with input
    // check result event and update channel db
}
