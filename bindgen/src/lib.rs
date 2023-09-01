use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn pure_wasm_bindgen(args: TokenStream, input: TokenStream) -> TokenStream {
    let codec_arg: u64 = 0x0200;
    let function = parse_macro_input!(input as ItemFn);
    let function_name = &function.sig.ident;
    let output = quote! {
        pub mod #function_name {
            mod inner {
                use super::*;
                #function
            }

            #[no_mangle]
            pub unsafe extern "C" fn #function_name(ptr: *mut u8, len: i64) -> (i64, i64) {
                let codec = match #codec_arg {
                    0x0200 => JsonCodec,
                    0x51 => CborCodec
                };
            }
        }
    };
    output.into()
}
