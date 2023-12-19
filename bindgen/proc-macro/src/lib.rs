use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, PatType};

#[proc_macro_attribute]
pub fn purewasm_bindgen(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let function_name = &function.sig.ident;
    if function.sig.inputs.len() != 1 {
        return TokenStream::from(quote! {
            compile_error!("There should be one(only) input parameter");
        });
    }

    let input_type = match &function.sig.inputs[0] {
        syn::FnArg::Typed(PatType { ty, .. }) => ty,
        _ => unreachable!()
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
                let memory = WasmMemory {
                    codec: purewasm::CodecImpl{},
                };
                let input: Result<#input_type, String> = memory.from_memory(ptr, len);
                if let Ok(input) = input {
                    let result = inner::#function_name(input);
                    memory.to_memory(result)
                } else {
                    let r: #output_type = Err("NO_INPUT".into());
                    return memory.to_memory(r);
                }
            }
        }
    };
    output.into()
}
