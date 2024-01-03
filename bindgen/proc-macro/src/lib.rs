use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, PatType};

#[proc_macro_attribute]
pub fn purewasm_bindgen(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let function_name = &function.sig.ident;
    
    if function.sig.inputs.len() != 2 {
        return TokenStream::from(quote! {
            compile_error!("Expected two arguments in the function signature");
        });
    }
    let input_type = match &function.sig.inputs[0] {
        syn::FnArg::Typed(PatType { ty, .. }) => ty,
        _ => unreachable!(),
    };

    let output = quote! {
        pub mod #function_name {
            use super::*;
            mod inner {
                use super::*;
                #function
            }

            #[no_mangle]
            pub unsafe extern "C" fn #function_name(ptr: *mut u8, len: i32) -> (i32, i32) {
                let handle_fn = |ptr: *mut u8, len: i32| -> Result<(), WasmError> {
                    unsafe {
                        let mut msg = core::slice::from_raw_parts(ptr, len as usize);
                        let mut payload_len_buf = [0u8; 4];
                        msg.read_exact(&mut payload_len_buf).unwrap();
                        let payload_len = u32::from_le_bytes(payload_len_buf);
                        let mut payload_buf: Vec<u8> = Vec::with_capacity(payload_len as usize);
                        msg.read_exact(&mut payload_buf).unwrap();
                        let payload: #input_type = CodecImpl::from_bytes(&payload_buf)?;
                        let mut signers: Vec<String> = Vec::new();
                        while msg.is_empty() == false {
                            let mut signer_len_buf = [0u8; 2];
                            msg.read_exact(&mut signer_len_buf).unwrap();
                            let signer_len = u16::from_le_bytes(signer_len_buf);
                            let mut signer_buf: Vec<u8> = Vec::with_capacity(signer_len as usize);
                            msg.read_exact(&mut signer_buf).unwrap();
                            signers.push(String::from_utf8(signer_buf).unwrap());
                        }
                        inner::#function_name(payload, signers)
                    }
                };
                let result = handle_fn(ptr, len);
                unsafe { drop(Box::from_raw(ptr)); }
                WasmMemory { codec: CodecImpl }.to_memory(result).unwrap()
            }
        }
    };
    output.into()
}

fn handle_fn(ptr: *mut u8, len: i32) -> Result<(), WasmError> {
    unsafe {
        let mut msg = core::slice::from_raw_parts(ptr, len as usize);
        let mut payload_len = &msg[..4];
        let payload_len = u32::from_le_bytes(payload_len.try_into().unwrap());
        let payload_slice = &msg[4..payload_len as usize + 4];
        let mut payload: Vec<u8> = Vec::with_capacity(payload_len as usize);

        msg.read_exact(&mut payload_buf).unwrap();
        let payload: String = String::from_utf8(signer_buf).unwrap();
        let mut signers: Vec<String> = Vec::new();
        while msg.is_empty() == false {
            let mut signer_len_buf = [0u8; 2];
            msg.read_exact(&mut signer_len_buf).unwrap();
            let signer_len = u16::from_le_bytes(signer_len_buf);
            let mut signer_buf: Vec<u8> = Vec::with_capacity(signer_len as usize);
            msg.read_exact(&mut signer_buf).unwrap();
            signers.push(String::from_utf8(signer_buf).unwrap());
        }
        Ok(())
        //inner::#function_name(payload, signers)
    }
}
/*
 let handle_fn = |ptr: *mut u8, len: i32| -> Result<(), WasmError> {
                    unsafe {
                        let msg = core::slice::from_raw_parts(ptr, len as usize);
                        
                        let msg: WasmsgParam =  
                            WasmMemory { codec: CborCodec }.from_memory(ptr, len)?;
                        let input: #input_type = CodecImpl::from_bytes(&msg.payload)?;
                        inner::#function_name(input, msg.signers)
                    }
                };
*/