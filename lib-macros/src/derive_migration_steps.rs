use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{DeriveInput, Expr, Field, Ident, Type};

pub fn from_derive_input(input: DeriveInput) -> TokenStream {
    let struct_ident = input.ident;
    let syn::Data::Struct(data_struct) = input.data else {
        panic!("expected struct");
    };
    let field_requirements = data_struct
        .fields
        .iter()
        .filter_map(|field| {
            field
                .attrs
                .iter()
                .filter(|attribute| attribute.path().is_ident("migration_requirement"))
                .map(|attribute| {
                    // this is assigned a name because rustfmt gives up when it's inline
                    type CommaSeparatedKeyValuePairs =
                        syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]>;
                    let args = attribute
                        .parse_args_with(CommaSeparatedKeyValuePairs::parse_terminated)
                        .expect("attribute arguments should be key=value separated by comma")
                        .into_iter()
                        .map(|name_value| {
                            (
                                name_value.path.get_ident().unwrap().to_string(),
                                name_value.value,
                            )
                        })
                        .collect::<HashMap<String, Expr>>();

                    (
                        field.ident.clone().unwrap(),
                        FieldRequirement {
                            required_field: args
                                .get("field")
                                .map(|field_expr| {
                                    format_ident!("{}", field_expr.to_token_stream().to_string())
                                })
                                .expect("field is required"),
                            required_value: args.get("value").cloned().expect("value is required"),
                        },
                    )
                })
                .next()
        })
        .collect::<HashMap<Ident, FieldRequirement>>();

    let struct_fields_enum = FieldEnum::new(&struct_ident, data_struct.fields.iter());
    let as_enum = data_struct.fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        struct_fields_enum.wrap_field(ident, &quote! { s.#ident })
    });
    let from_enum = data_struct.fields.iter().enumerate().map(|(index, field)| {
        let ident = field.ident.as_ref().unwrap();
        let unwrapped = struct_fields_enum.unwrap_field(ident, index);
        quote! { #ident: #unwrapped }
    });

    let migration_planner_args = data_struct.fields.iter().map(|field| {
        match field_requirements.get(field.ident.as_ref().unwrap()) {
            Some(field_requirement) => {
                let (requirement_index, requirement) = data_struct
                    .fields
                    .iter()
                    .enumerate()
                    .find(|(_, f)| *f.ident.as_ref().unwrap() == field_requirement.required_field)
                    .expect("required field does not exist");
                let requirement_value = &field_requirement.required_value;
                let requirement_variant_ident = format_ident!(
                    "{}",
                    heck::AsUpperCamelCase(requirement.ident.as_ref().unwrap().to_string())
                        .to_string()
                );
                let wrapped = struct_fields_enum.wrap_field(
                    &requirement_variant_ident,
                    &requirement_value.to_token_stream(),
                );
                // rustfmt gives up when the full path is specified here rather than just Requirement. Delete up to requirement,
                // run rustfmt, and then put it back.
                quote! {
                    Some(::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::Requirement {
                        index: #requirement_index,
                        value: #wrapped,
                    })
                }
            }
            None => quote! { None },
        }
    });

    let struct_fields_enum_ident = &struct_fields_enum.ident;
    let size = data_struct.fields.len();

    quote! {
        #struct_fields_enum

        impl ::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::Migrate<#size> for #struct_ident {
            type T = #struct_fields_enum_ident;

            fn migrate(
                migration_planner: &::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::MigrationPlanner<Self::T, #size>,
                from: &Self,
                to: &Self
            ) -> Vec<Self> {
                fn as_byte_array(s: &#struct_ident) -> [#struct_fields_enum_ident; #size] {
                    [
                        #(#as_enum,)*
                    ]
                }

                fn from_byte_array(data: [#struct_fields_enum_ident; #size]) -> #struct_ident {
                    #struct_ident {
                        #(#from_enum,)*
                    }
                }

                migration_planner.migrate(as_byte_array(from), &as_byte_array(to))
                    .into_iter()
                    .map(from_byte_array)
                    .collect::<Vec<Self>>()
            }

            fn migration_planner() -> ::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::MigrationPlanner<Self::T, #size> {
                ::openscq30_lib::devices::soundcore::common::modules::sound_modes_v2::MigrationPlanner::new([
                    #(#migration_planner_args,)*
                ])
            }
        }
    }
}

struct FieldEnum {
    ident: Ident,
    fields: Vec<(Ident, Type)>,
}

impl FieldEnum {
    pub fn new<'a>(
        from_struct_ident: &Ident,
        from_struct_fields: impl IntoIterator<Item = &'a Field>,
    ) -> Self {
        let ident = format_ident!("{}Fields", from_struct_ident);
        let fields = from_struct_fields
            .into_iter()
            .map(|field| {
                let variant_ident =
                    Self::field_ident_to_variant_ident(&field.ident.clone().unwrap());
                let ty = field.ty.clone();
                (variant_ident, ty)
            })
            .collect::<Vec<_>>();

        Self { ident, fields }
    }

    fn wrap_field(&self, ident: &Ident, inner: &TokenStream) -> TokenStream {
        let enum_ident = &self.ident;
        let variant_ident = Self::field_ident_to_variant_ident(&ident);
        quote! { #enum_ident::#variant_ident(#inner) }
    }

    fn unwrap_field(&self, ident: &Ident, index: usize) -> TokenStream {
        let enum_ident = &self.ident;
        let variant_ident = Self::field_ident_to_variant_ident(&ident);
        quote! {
            match data[#index] {
                #enum_ident::#variant_ident(value) => value,
                _ => unreachable!("the variant is being taken from the same index it was put into, and it shuoldn't have changed"),
            }
        }
    }

    pub fn field_ident_to_variant_ident(field_ident: &Ident) -> Ident {
        format_ident!(
            "{}",
            heck::AsUpperCamelCase(field_ident.to_string()).to_string()
        )
    }
}

impl ToTokens for FieldEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let variants = self.fields.iter().map(|(field_ident, field_type)| {
            quote! { #field_ident(#field_type) }
        });
        tokens.extend(quote! {
            #[derive(PartialEq, Eq, Copy, Clone, Hash)]
            pub enum #ident {
                #(#variants,)*
            }
        });
    }
}

struct FieldRequirement {
    required_field: Ident,
    required_value: Expr,
}
