use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Entity)]
pub fn entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.to_string();

    let tokens = quote! {
        impl #struct_name {
            pub fn read() -> String {
                String::from("Hello, world!")
            }
        }
    };

    tokens.into()
}