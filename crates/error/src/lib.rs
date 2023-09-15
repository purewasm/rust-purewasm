#![cfg_attr(not(test), no_std)]
extern crate alloc;
use alloc::{borrow::ToOwned, collections::BTreeMap, string::String};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ErrorValue {
    Boolean(bool),
    Number(i32),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PureError {
    code: String,
    details: BTreeMap<String, ErrorValue>,
}

impl PureError {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
            details: BTreeMap::new(),
        }
    }

    pub fn add_str(&mut self, key: &str, value: &str) -> &mut Self {
        self.details
            .insert(key.to_owned(), ErrorValue::String(value.to_owned()));
        self
    }

    pub fn add_i32(&mut self, key: &str, value: i32) -> &mut Self {
        self.details
            .insert(key.to_owned(), ErrorValue::Number(value));
        self
    }

    pub fn add_bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.details
            .insert(key.to_owned(), ErrorValue::Boolean(value));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_error_new() {
        // Test the PureError::new method
        let pure_error = PureError::new("new_error");

        assert_eq!(pure_error.code, "new_error");
        assert!(pure_error.details.is_empty());
    }

    #[test]
    fn test_pure_error_add_str() {
        // Test the PureError::add_str method
        let mut pure_error = PureError::new("custom_error");
        pure_error.add_str("message", "Error message");

        assert_eq!(pure_error.details.len(), 1);
        match pure_error.details.get("message").unwrap() {
            ErrorValue::String(s) => assert_eq!(s, "Error message"),
            _ => panic!("Test failure")
        }
    }

    #[test]
    fn test_pure_error_add_i32() {
        // Test the PureError::add_i32 method
        let mut pure_error = PureError::new("custom_error");
        pure_error.add_i32("code", 404);

        assert_eq!(pure_error.details.len(), 1);
        match pure_error.details.get("code").unwrap() {
            ErrorValue::Number(n) => assert_eq!(*n, 404),
            _ => panic!("Test failure")
        }
    }

    #[test]
    fn test_pure_error_add_bool() {
        // Test the PureError::add_bool method
        let mut pure_error = PureError::new("custom_error");
        pure_error.add_bool("flag", true);

        assert_eq!(pure_error.details.len(), 1);
        match pure_error.details.get("flag").unwrap() {
            ErrorValue::Boolean(b) => assert_eq!(*b, true),
            _ => panic!("Test failure")
        }
    }
}
