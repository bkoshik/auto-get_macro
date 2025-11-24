use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Fields, Ident, ItemStruct, Type, Visibility};

pub fn builder_impl(input: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(input as ItemStruct);

    let name: Ident = input.ident;
    let name_builder: Ident = Ident::new(
        &format!("{}Builder", name),
        Span::call_site().into()
    );
    let name_builder_error: Ident = Ident::new(
        &format!("{}BuilderError", name),
        Span::call_site().into()
    );
    let vis: Visibility = input.vis;
    let output: TokenStream = match &input.fields {
        Fields::Named(fields) => {
            let result: Vec<(_, _, _, _)> = fields.named.iter().map(|f| {
                let f_name: Ident = f.ident.clone().unwrap();
                let f_name_not_found: Ident = Ident::new(
                    &format!("{}NotFound", &snake_to_pascal(&f_name.to_string())),
                    Span::call_site().into()
                );
                let f_type: &Type = &f.ty;
                let f_vis: &Visibility = &f.vis;

                let builder_struct_fields: _ = quote! {
                    #f_vis #f_name: Option<#f_type>
                };
                let builder_methods: _ = quote! {
                    pub fn #f_name<T: Into<#f_type>>(&mut self, value: T) -> &mut Self {
                        self.#f_name = Some(value.into());

                        self
                    }
                };
                let builder_create_fields: _ = quote! {
                    #f_name: match self.#f_name {
                        Some(ref value) => Clone::clone(value),
                        None => return Err(#name_builder_error::#f_name_not_found)
                    }
                };
                let builder_error_fields: _ = quote! {
                    #f_name_not_found
                };

                (builder_struct_fields, builder_methods, builder_create_fields, builder_error_fields)
            }).collect();

            let builder_struct_fields: Vec<_> = result.iter().map(|(f, _, _, _)| f).collect();
            let builder_methods: Vec<_> = result.iter().map(|(_, f, _, _)| f).collect();
            let builder_create_fields: Vec<_> = result.iter().map(|(_, _, f, _)| f).collect();
            let builder_error_fields: Vec<_> = result.iter().map(|(_, _, _, f)| f).collect();

            quote! {
                #[derive(Debug)]
                #vis enum #name_builder_error {
                    #(#builder_error_fields),*
                }

                impl std::fmt::Display for #name_builder_error {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self)
                    }
                }

                impl std::error::Error for #name_builder_error {}

                impl #name {
                    pub fn builder() -> #name_builder {
                        #name_builder::default()
                    }
                }

                #[derive(Default, Clone)]
                #vis struct #name_builder {
                    #(#builder_struct_fields),*
                }

                impl #name_builder {
                    #(#builder_methods)*
                }

                impl #name_builder {
                    pub fn build(&self) -> Result<#name, #name_builder_error> {
                        Ok(
                            #name {
                                #(#builder_create_fields),*
                            }
                        )
                    }
                }
            }.into()
        }
        _ => unimplemented!("Builder can only be derived for structs with named fields"),
    };

    output
}

fn snake_to_pascal(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in s.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    result
}
