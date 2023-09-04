#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

/*extern crate alloc;
use alloc::{borrow::ToOwned, format, string::String};

mod version1 {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Event(Vec<u8>, Vec<u8>);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct EventId(Vec<u8>);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct State {
        id: Vec<u8>,
    }
}

mod version2 {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Event(Vec<u8>, Vec<u8>);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct EventId(Vec<u8>);

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct State {
        id: Vec<u8>,
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum IdState {
    Version1(version1::State),
    Version2(version2::State),
}

pub struct Input {
    state: IdState,
    payload: Payload,
}

pub fn state(input: Input) -> Result<IdState> {
    IdState::Version2(Version2State)
}*/

use core::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Result {
    pub(crate) id: Vec<u8>,
}




