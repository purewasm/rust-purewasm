// /wasmid/<multihash(digest_id)>
// let re = Regex::new(r"(/wasmid/)-(?<id>A-Za-z0-9+)").unwrap();

// when a client wants to resolve an id 
pub fn resolve(id: &str) {
    // get id from id store
    // if exists return state
    // else let id: Vec<u8> = multibase.decode(id.split(/wasmid/{}))
    // build a gossip message 
    // create a channel to accept replies
}

// when a gossip replies resolve request
pub fn handle_resolve_reply() {

}

// when get a resolve request 
pub fn reply_resolve(id: &str) {

}

// when a client create an id
pub fn create() {
    // Insert it to inception pool
}

// when a client update an id
pub fn update() {
    // Insert it to mutation pool
}

// when a block 
pub fn send_to_notary(){

}

// when a notary commit block
pub fn handle_commit() {

}