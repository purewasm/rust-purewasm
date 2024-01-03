pub struct WasmsgProof {

}
#[no_mangle]
pub unsafe extern "C" fn verify(ptr: *mut u8, len: i32) -> (i32, i32) {
    let handle_fn = |ptr: *mut u8, len: i32| -> Result<Vec<String>, WasmError> {
        unsafe {
            let proof: WasmsgProof =  
                WasmMemory { codec: CodecImpl }.from_memory(ptr, len)?;
            get!("/users/{}", proof)
        }
    };
    let result = handle_fn(ptr, len);
    unsafe { drop(Box::from_raw(ptr)); }
    WasmMemory { codec: CodecImpl }.to_memory(result).unwrap()
}