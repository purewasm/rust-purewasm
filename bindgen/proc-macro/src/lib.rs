use proc_macro::TokenStream;
use quote::quote;
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

    let output_type = match &function.sig.output {
        syn::ReturnType::Type(_, ty) => ty,
        _ => unreachable!()
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
                let handle_fn = |ptr: *mut u8, len: i32| -> #output_type {
                    unsafe {
                        let cbor_memory = WasmMemory {
                            codec: CborCodec,
                        };
                        let msg: WasmsgParam = cbor_memory.from_memory(ptr, len)?;
                        let input: #input_type = CodecImpl::from_bytes(&msg.input)?;
                        inner::#function_name(input, msg.signers)
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
