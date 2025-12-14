use syn::{DeriveInput, parse_macro_input};

use crate::derive_has::DeriveHas;

mod derive_has;
mod derive_migration_steps;

#[proc_macro_derive(Has, attributes(has))]
pub fn derive_has(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let derive_has_trait_impl = DeriveHas::from_derive_input(derive_input);
    quote::quote! {
        #derive_has_trait_impl
    }
    .into()
}

#[proc_macro_derive(MigrationSteps, attributes(migration))]
pub fn derive_migration_steps(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    derive_migration_steps::from_derive_input(derive_input).into()
}
