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
                        let msg = core::slice::from_raw_parts(ptr, len as usize);
                        let payload_len = &msg[..4];
                        let payload_len = u32::from_le_bytes(payload_len.try_into().unwrap());
                        let payload = msg[4..payload_len as usize + 4].to_vec();
                        let payload: #input_type = CodecImpl::from_bytes(&payload)?;

                        let mut signers = Vec::new();
                        let mut index = payload_len as usize + 4;

                        while index < msg.len() {
                            let length_bytes = &msg[index..index + 4];
                            let length = u32::from_le_bytes(length_bytes.try_into().unwrap()) as usize;

                            index += 4;

                            let slice = msg[index..index + length].to_vec();
                            signers.push(String::from_utf8(slice).unwrap());

                            index += length;
                        }
                        inner::#function_name(payload, signers)
                    }
                };
                let result = handle_fn(ptr, len);
                unsafe { drop(Box::from_raw(ptr)); }
                match result {
                    Ok(_) => (0, 0),
                    Err(err) => {
                        let mut err_str = err.code;
                        if let Some(message) = err.message {
                            err_str = format!("{} - {}", err_str, message);
                        }
                        return (err_str.as_ptr() as i32, err_str.len() as i32);
                    },
                }
            }
        }
    };
    output.into()
}
