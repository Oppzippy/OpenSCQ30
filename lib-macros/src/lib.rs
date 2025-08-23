use syn::{DeriveInput, parse_macro_input};

use crate::derive_has::DeriveHas;

mod derive_has;

#[proc_macro_derive(Has, attributes(has, maybe_has))]
pub fn derive_has(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let derive_has_trait_impl = DeriveHas::from_derive_input(derive_input);
    quote::quote! {
        #derive_has_trait_impl
    }
    .into()
}
