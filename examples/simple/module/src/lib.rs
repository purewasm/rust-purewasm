#![no_std]
#![no_main]

purewasm_core::use_purewasm!();

#[no_mangle]
pub unsafe extern "C" fn xyz() -> i32 {
    6
}
