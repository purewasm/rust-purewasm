use alloc::{borrow::ToOwned, collections::BTreeMap, string::String};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ErrorValue {
    Boolean(bool),
    Number(i32),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct Error<T: Serialize> {
    pure: PureError,
    custom: Option<T>
}

#[derive(Serialize, Deserialize)]
struct PureError {
    code: String,
    details: BTreeMap<String, ErrorValue>,
}

impl <T> Error<T> where T : Serialize {
    pub fn new(code: &str) -> Self {
        let pure = PureError {
            code: code.to_owned(),
            details: BTreeMap::new(),
        };
        Self { pure: pure, custom: None }
    }
}