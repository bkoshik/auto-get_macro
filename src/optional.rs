use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Field, Fields, Ident, ItemStruct, Type, Visibility};
use syn::__private::TokenStream2;

pub fn optional_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(item as ItemStruct);

    let name: Ident = input.ident;
    let vis: &Visibility = &input.vis;
    let attrs: Vec<_> = input.attrs.iter().filter(|attr| !attr.path().is_ident("optional")).collect();
    let output: TokenStream = match &input.fields {
        Fields::Named(fields) => {
            let result: Vec<_> = fields.named.iter().map(|f| {
                parse_field(&f)
            }).collect();

            quote! {
                #(#attrs)*
                #vis struct #name {
                    #(#result),*
                }
            }.into()
        }
        Fields::Unnamed(fields) => {
            let result: Vec<_> = fields.unnamed.iter().map(|f| {
                let f_type: &Type = &f.ty;

                quote! {
                    Option<#f_type>
                }
            }).collect();

            quote! {
                #(#attrs)*
                #vis struct #name(#(#result),*);
            }.into()
        }
        _ => unimplemented!("Optional can only be derived for structs with named and unnamed fields"),
    };

    output
}

fn parse_field(f: &Field) -> TokenStream2 {
    let f_name: &Ident = f.ident.as_ref().unwrap();
    let f_type: &Type = &f.ty;
    let f_vis: &Visibility = &f.vis;

    let mut is_except = false;

    for attr in &f.attrs {
        if attr.path().is_ident("optional")
            && let Ok(nested) = attr.parse_args::<syn::Ident>()
            && nested == "except"
        {
            is_except = true
        }
    }

    if is_except {
        quote! {
            #f_vis #f_name: #f_type
        }
    } else {
        quote! {
            #f_vis #f_name: Option<#f_type>
        }
    }
}
