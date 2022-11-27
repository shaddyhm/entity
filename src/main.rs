use std::str::FromStr;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemStruct;

fn main() {
    let s = "struct Animal { name: String, kind: String }";
    let tokens = TokenStream::from_str(s).unwrap();
    let ast: ItemStruct = syn::parse2(tokens).unwrap();
    let struct_ident = ast.ident;
    let fields: Vec<String> = ast.fields
        .into_iter()
        .map(|field| field.ident.unwrap().to_string())
        .collect();
    let statement = format!("SELECT {} FROM {};", fields.join(", "), format_ident!("{}", struct_ident).to_string());
    let output = quote! {
        impl #struct_ident {
            pub fn read() -> &str {
                #statement
            }
        }
    };
    println!("{}", output);
}