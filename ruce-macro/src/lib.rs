use std::fs;
use std::io::Write;

use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, ItemFn, Ident, spanned::Spanned};
use quote::quote;

mod init_file;

use init_file::initialize_file;

static mut FUNCTION_ID_COUNTER: u32 = 0;

macro_rules! unique_function {
    ($func_name:ident) => {
        {
            unsafe {
                FUNCTION_ID_COUNTER += 1;
                format!("__{}_{}", $func_name, FUNCTION_ID_COUNTER)
            }
        }
    }
}

#[proc_macro_attribute]
pub fn js_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    
    let file_path = initialize_file("js_function.c");

    let input = parse_macro_input!(input as ItemFn); 

    let function_name = &input.sig.ident;
    let function_name_at_c = unique_function!(function_name);
    let function_name_at_c = Ident::new(&function_name_at_c, input.span());

    let block = &input.block.stmts;

    let args = &input.sig.inputs.iter().collect::<Vec<_>>();

    let params = args.iter().map(|arg| {
        let arg = match arg {
            syn::FnArg::Typed(arg) => arg,
            _ => panic!("Unsupported argument type")
        };

        arg.pat.clone()
    }).collect::<Vec<_>>();

    let c_args = args.iter().map(|arg| {
        let arg = match arg {
            syn::FnArg::Typed(arg) => arg,
            _ => panic!("Unsupported argument type")
        };

        let pat = &arg.pat;
        let ty = &arg.ty.span().source_text().unwrap();

        let c_type = match ty.as_str() {
            "i32" => quote! { int },
            "f32" => quote! { float },
            "f64" => quote! { double },
            _ => panic!("Unsupported argument type")
        };

        quote! {
            #c_type #pat
        }
    }).collect::<Vec<_>>();

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();

    let function_code = quote! {
        EM_JS(void, #function_name_at_c, (#(#c_args),*), {
            #(#block)*
        });
    };

    write!(file, "{}\n", function_code).unwrap();

    let expanded = quote! {
        extern "C" {
            pub fn #function_name_at_c(#(#args),*);
        }

        pub fn #function_name(#(#args),*) {
            unsafe {
                #function_name_at_c(#(#params),*);
            }
        }
    };

    return expanded.into();
}
