use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, GenericArgument, Ident, PathArguments, Type};

pub struct DeriveHas {
    fields: Vec<TraitFieldData>,
}

impl DeriveHas {
    pub fn from_derive_input(input: DeriveInput) -> Self {
        let struct_ident = input.ident;
        let variants = match input.data {
            syn::Data::Struct(data_struct) => data_struct
                .fields
                .into_iter()
                .filter_map(|field| {
                    field
                        .attrs
                        .iter()
                        .filter_map(|attribute| {
                            if attribute.path().is_ident("has") {
                                Some(TraitFieldData {
                                    struct_ident: struct_ident.clone(),
                                    field_ident: field
                                        .ident
                                        .clone()
                                        .expect("missing identifier for field"),
                                    field_type: field.ty.clone(),
                                    is_maybe: false,
                                })
                            } else if attribute.path().is_ident("maybe_has") {
                                let option_inner_type = if let Type::Path(type_path) = &field.ty {
                                    let type_params = &type_path.path.segments[0].arguments;
                                    if let PathArguments::AngleBracketed(params) = type_params {
                                        if let GenericArgument::Type(ty) = &params.args[0] {
                                            ty
                                        } else {
                                            panic!("expected a type in angle brackets")
                                        }
                                    } else {
                                        panic!("expected angle bracketed args")
                                    }
                                } else {
                                    panic!("expected path")
                                };
                                Some(TraitFieldData {
                                    struct_ident: struct_ident.clone(),
                                    field_ident: field
                                        .ident
                                        .clone()
                                        .expect("missing identifier for field"),
                                    field_type: option_inner_type.clone(),
                                    is_maybe: true,
                                })
                            } else {
                                None
                            }
                        })
                        .next()
                })
                .collect(),
            _ => panic!("expected struct"),
        };
        Self { fields: variants }
    }
}

impl ToTokens for DeriveHas {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { fields } = self;
        tokens.extend(quote! {
            #(#fields)*
        });
    }
}

struct TraitFieldData {
    struct_ident: Ident,
    field_ident: Ident,
    field_type: Type,
    is_maybe: bool,
}

impl ToTokens for TraitFieldData {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_maybe {
            self.maybe_has_tokens(tokens);
        } else {
            self.has_tokens(tokens);
        }
    }
}

impl TraitFieldData {
    fn has_tokens(&self, tokens: &mut TokenStream) {
        let TraitFieldData {
            struct_ident,
            field_ident,
            field_type,
            is_maybe: _,
        } = &self;

        tokens.extend(quote! {
            impl openscq30_lib_has::Has<#field_type> for #struct_ident {
                fn get(&self) -> &#field_type {
                    &self.#field_ident
                }

                fn get_mut(&mut self) -> &mut #field_type {
                    &mut self.#field_ident
                }
            }
        });
    }

    fn maybe_has_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let TraitFieldData {
            struct_ident,
            field_ident,
            field_type,
            is_maybe: _,
        } = &self;

        tokens.extend(quote! {
            impl openscq30_lib_has::MaybeHas<#field_type> for #struct_ident {
                fn maybe_get(&self) -> Option<&#field_type> {
                    self.#field_ident.as_ref()
                }

                fn maybe_get_mut(&mut self) -> Option<&mut #field_type> {
                    self.#field_ident.as_mut()
                }

                fn set_maybe(&mut self, maybe_value: Option<#field_type>) {
                    self.#field_ident = maybe_value;
                }
            }
        });
    }
}
