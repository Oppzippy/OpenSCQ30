use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Expr, Ident};

pub fn from_derive_input(input: DeriveInput) -> TokenStream {
    let struct_ident = input.ident;
    let syn::Data::Struct(data_struct) = input.data else {
        panic!("expected struct");
    };
    let fields = data_struct
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            field
                .attrs
                .iter()
                .filter(|attribute| attribute.path().is_ident("migration"))
                .map(|attribute| {
                    let args = attribute
                        .parse_args_with(syn::punctuated::Punctuated::<
                            syn::MetaNameValue,
                            syn::Token![,],
                        >::parse_terminated)
                        .expect("attribute arguments should be key=value separated by comma")
                        .into_iter().map(|name_value| {
                            (name_value.path.get_ident().unwrap().to_string(), name_value.value)
                        })
                        .collect::<HashMap<String, Expr>>();

                    FieldWithArguments {
                        ident: field.ident.clone().unwrap(),
                        from: args.get("from").expect("from is required").clone(),
                        to: args.get("to").expect("to is required").clone(),
                        required_field: args.get("required_field").cloned(),
                        required_value: args.get("required_value").cloned(),
                        index,
                    }
                })
                .next()
                .expect("every field should have a #[migrate(...)] attribute")
        })
        .collect::<Vec<FieldWithArguments>>();

    let size = data_struct.fields.len();
    let as_byte_array_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        let to_u8 = &field.to;
        quote! { (#to_u8)(s.#ident) }
    });
    let from_byte_array_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        let from_u8 = &field.from;
        let index = field.index;
        quote! { #ident: (#from_u8)(data[#index]) }
    });
    let migration_planner_args = fields.iter().map(|field| match &field.required_field {
        Some(field_name_expr) => {
            let field_name = field_name_expr.into_token_stream().to_string();
            let requirement = fields
                .iter()
                .find(|f| f.ident.to_string() == field_name)
                .expect("required field does not exist");
            let requirement_index = requirement.index;
            let requirement_to_byte = &requirement.to;
            let requirement_value = field
                .required_value
                .as_ref()
                .expect("if required_field is set, required_value must also be set");
            quote! {
                Some(::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::Requirement {
                    index: #requirement_index,
                    value: (#requirement_to_byte)(#requirement_value),
                })
            }
        }
        None => quote! { None },
    });
    quote! {
        impl ::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::Migrate<#size> for #struct_ident {
            fn migrate(
                migration_planner: &::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::MigrationPlanner<#size>,
                from: &Self,
                to: &Self
            ) -> Vec<Self> {
                fn as_byte_array(s: &#struct_ident) -> [u8; #size] {
                    [
                        #(#as_byte_array_fields,)*
                    ]
                }

                fn from_byte_array(data: [u8; #size]) -> #struct_ident {
                    #struct_ident {
                        #(#from_byte_array_fields,)*
                    }
                }

                migration_planner.migrate(as_byte_array(from), &as_byte_array(to))
                    .into_iter()
                    .map(from_byte_array)
                    .collect::<Vec<Self>>()
            }

            fn migration_planner() -> ::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::MigrationPlanner<#size> {
                ::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::MigrationPlanner::new([
                    #(#migration_planner_args,)*
                ])
            }
        }
    }
}

struct FieldWithArguments {
    ident: Ident,
    from: Expr,
    to: Expr,
    required_field: Option<Expr>,
    required_value: Option<Expr>,
    index: usize,
}
