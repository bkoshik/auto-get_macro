//! This crate provides procedural macros:
//! - `#[derive(AutoGetters)]`
//! - `#[optional]`
//!
//! They're helping for create structures without lots of code
//!
//! <br>
//!
//! # Example
//!
//! ```
//! use fast_struct::AutoGetters;
//!
//! #[derive(AutoGetters)]
//! struct Foo {
//!     bar: i16,
//!     baz: String,
//! }
//! ```
//! 
//! will generate:
//! 
//! ```
//! struct Foo {
//!     bar: i16,
//!     baz: String,
//! }
//!
//! impl Foo {
//!     pub fn bar(&self) -> &i16 {
//!         &self.bar
//!     }
//!
//!     pub fn baz(&self) -> &String {
//!         &self.baz
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Type, Visibility, ItemStruct, Fields};

/// Automatically generates ***getter methods*** only for **Named Structures**
/// 
/// # Example
///
/// ```
/// use fast_struct::AutoGetters;
///
/// #[derive(AutoGetters)]
/// struct Foo {
///     bar: i16,
///     baz: String,
/// }
/// ```
///
/// will implement these methods for `Foo`:
///
/// ```
/// struct Foo {
///     bar: i16,
///     baz: String,
/// }
///
/// impl Foo {
///     pub fn bar(&self) -> &i16 {
///         &self.bar
///     }
///
///     pub fn baz(&self) -> &String {
///         &self.baz
///     }
/// }
/// ```
#[proc_macro_derive(AutoGetters)]
pub fn auto_getters(input: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(input as ItemStruct);

    let name: Ident = input.ident;
    let output: TokenStream = match &input.fields {
        Fields::Named(fields) => {
            let result: Vec<_> = fields.named.iter().map(|f| {
                let f_name: &Ident = f.ident.as_ref().unwrap();
                let f_type: &Type = &f.ty;

                quote! {
                    pub fn #f_name(&self) -> &#f_type {
                        &self.#f_name
                    }
                }
            }).collect();

            quote! {
                impl #name {
                    #(#result)*
                }
            }.into()
        }
        _ => unimplemented!("AutoGetters can only be derived for structs with named fields"),
    };

    output
}

/// Makes all fields of **Named/Unnamed Structures** optional
///
/// Structure`s fields accept attributes:
/// - `#[except]` for leave as is
///
/// # Example
///
/// ```
/// use fast_struct::optional;
///
/// #[optional]
/// pub struct Foo {
///     #[except]
///     bar: bool,
///     baz: usize,
///     qux: String,
///     quux: Vec<String>,
/// }
/// ```
///
/// will generate:
///
/// ```
/// pub struct Foo {
///     bar: bool,
///     baz: Option<usize>,
///     qux: Option<String>,
///     quux: Option<Vec<String>>
/// }
/// ```
#[proc_macro_attribute]
pub fn optional(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(item as ItemStruct);

    let name: Ident = input.ident;
    let vis: &Visibility = &input.vis;
    let attrs: Vec<_> = input.attrs.iter().filter(|attr| !attr.path().is_ident("optional")).collect();
    let result: TokenStream = match &input.fields {
        Fields::Named(fields) => {
            let result: Vec<_> = fields.named.iter().map(|f| {
                let f_name: &Ident = f.ident.as_ref().unwrap();
                let f_type: &Type = &f.ty;
                let f_vis: &Visibility = &f.vis;

                let mut is_except = false;

                for attr in &f.attrs {
                    let path = attr.path();

                    if path.is_ident("except") {
                        is_except = true;
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
                    Option<&#f_type>
                }
            }).collect();

            quote! {
                #(#attrs)*
                #vis struct #name(#(#result),*);
            }.into()
        }
        _ => unimplemented!("Optional can only be derived for structs with named and unnamed fields"),
    };

    result
}
