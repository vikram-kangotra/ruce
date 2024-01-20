use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn js_code(attr: TokenStream, input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as ItemFn);

    let function_name = input.sig.ident;

    let expanded = quote! {
        #[no_mangle]
        extern "C" {
            pub fn #function_name();
        }
    };

    return expanded.into();
}
