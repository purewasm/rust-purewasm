use alloc::string::String;
use serde::{de::DeserializeOwned, Serialize};

pub trait Store {
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, String>;

    fn put<T: Serialize>(&self, key: &str, t: &T) -> Result<(), String>;
}
