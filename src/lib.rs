use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(AutoGet)]
pub fn auto_get_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: Ident = input.ident;

    let fields: Fields = if let syn::Data::Struct(data) = input.data {
        data.fields
    } else {
        return TokenStream::new()
    };

    let getters = fields.iter().map(|f| {
        let f_name: &Ident = f.ident.as_ref().unwrap();
        let f_type: &Type = &f.ty;

        quote! {
            pub fn #f_name(&self) -> &#f_type {
                &self.#f_name
            }
        }
    });

    let expanded: TokenStream = quote! {
        impl #name {
            #(#getters)*
        }
    }.into();

    expanded
}