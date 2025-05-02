use heck::ToKebabCase;
use proc_macro2::Ident;
use quote::{ToTokens, quote};
use syn::{Attribute, DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(Translate, attributes(translate))]
pub fn derive_translate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let translatable_enum = TranslatableEnum::from_derive_input(derive_input);
    quote::quote! {
        #translatable_enum
    }
    .into()
}

struct TranslatableEnum {
    ident: Ident,
    variants: Vec<TranslatableEnumVariant>,
}

impl TranslatableEnum {
    pub fn from_derive_input(input: DeriveInput) -> Self {
        let enum_ident = input.ident;
        let variants = match input.data {
            syn::Data::Enum(data_enum) => data_enum
                .variants
                .iter()
                .map(|variant| TranslatableEnumVariant {
                    enum_ident: enum_ident.to_owned(),
                    variant_ident: variant.ident.to_owned(),
                    translation_key: variant
                        .attrs
                        .iter()
                        .filter_map(TranslatableEnumAttribute::from_attribute)
                        .next()
                        .map(|attr| attr.key)
                        .unwrap_or_else(|| format!("{}", variant.ident).to_kebab_case()),
                })
                .collect::<Vec<_>>(),
            _ => panic!("expected enum"),
        };
        Self {
            ident: enum_ident,
            variants,
        }
    }
}

struct TranslatableEnumAttribute {
    key: String,
}

impl TranslatableEnumAttribute {
    pub fn from_attribute(attribute: &Attribute) -> Option<Self> {
        if attribute.path().is_ident("translate") {
            let key = attribute.parse_args::<LitStr>().unwrap();
            return Some(Self { key: key.value() });
        }
        None
    }
}

impl ToTokens for TranslatableEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { ident, variants } = self;
        tokens.extend(quote! {
            impl ::openscq30_i18n::Translate for #ident {
                fn translate(&self) -> String {
                    match self {
                        #(#variants)*
                    }
                }
            }
        })
    }
}

struct TranslatableEnumVariant {
    enum_ident: Ident,
    variant_ident: Ident,
    translation_key: String,
}

impl ToTokens for TranslatableEnumVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            enum_ident,
            variant_ident,
            translation_key,
        } = self;
        tokens.extend(quote! {
            #enum_ident::#variant_ident => crate::i18n::fl!(#translation_key),
        });
    }
}
