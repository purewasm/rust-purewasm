#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Event(pub Vec<u8>, pub Vec<u8>);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EventId(pub Vec<u8>);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IdInput {
    state: Option<WrappedResult>,
    payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum WrappedResult {
    Version1(Version1Result),
    Version2(Version2Result),
}

pub fn handle(input: IdInput) -> Result<WrappedResult>{
    // logic
}