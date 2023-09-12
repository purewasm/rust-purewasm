#![no_main]
#![cfg_attr(not(test), no_std)]
mod model;
mod impls;
use purewasm_core::use_purewasm;
use_purewasm!();