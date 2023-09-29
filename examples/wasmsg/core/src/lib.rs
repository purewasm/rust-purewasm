use alloc::{collections::BTreeMap, vec::Vec};

pub trait KeyValueTx {
    /*fn get_context(id: &[u8]) -> Result<Vec<u8>, ()>;
    fn get_channel(ch: &str) -> Result<Channel, ()>;
    fn get_last_message(ch: &str) -> Result<PureMessage, ()>;
    fn get_values(ch: &str, keys: Vec<String>) -> BTreeMap<String, Vec<u8>>;
    fn put_message(msg: PureMessage, events: BTreeMap<String, Vec<u8>>) -> bool;*/
    fn get<T>(key: &str) -> Result<T, DbError>; 
    fn put<T>(key: &str, value: &T) -> Result<(), DbError>;
}

pub struct MemoryKeyValueTx {
   pub db: Arc<Mutex<HashMap<String, Vec<u8>>>> 
}

