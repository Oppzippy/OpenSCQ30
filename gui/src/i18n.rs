use std::{str::FromStr, sync::LazyLock};

use i18n_embed::{
    DefaultLocalizer, LanguageLoader, Localizer,
    fluent::{FluentLanguageLoader, fluent_language_loader},
    unic_langid::LanguageIdentifier,
};
use rust_embed::RustEmbed;

// libcosmic crashes when rendering hebrew text in some contexts, so it should be disabled until that's fixed
pub fn is_language_enabled(identifier: &LanguageIdentifier) -> bool {
    identifier.language.as_str() != "he"
}

/// Applies the requested language(s) to requested translations from the `fl!()` macro.
pub fn init(requested_languages: &[LanguageIdentifier]) {
    if let Err(why) = localizer().select(requested_languages) {
        eprintln!("error while loading fluent localizations: {why}");
    }
}

// Get the `Localizer` to be used for localizing this library.
#[must_use]
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub fn languages() -> impl Iterator<Item = (LanguageIdentifier, String)> {
    language_identifiers()
        .filter(is_language_enabled)
        .map(|identifier| {
            let name = language_name(&identifier);
            (identifier, name)
        })
}

fn language_identifiers() -> impl Iterator<Item = LanguageIdentifier> {
    Localizations::iter()
        .map(|path| {
            path.split_once('/')
                .map(|it| it.0.to_string())
                .unwrap_or(path.to_string())
        })
        .map(|language| {
            LanguageIdentifier::from_str(&language)
                .expect("language dirs should be valid language identifiers")
        })
}

// TODO consider doing this at build time so we don't need to load every single language at runtime
fn language_name(identifier: &LanguageIdentifier) -> String {
    let loader = FluentLanguageLoader::new("openscq30-gui", identifier.to_owned());
    loader
        .load_languages(&Localizations, &[identifier.to_owned()])
        .unwrap();
    if loader.has("language-name") {
        loader.get("language-name")
    } else {
        identifier.to_string()
    }
}

pub static LANGUAGE_LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader
        .load_fallback_language(&Localizations)
        .expect("Error while loading fallback language");

    loader
});

/// Request a localized string by ID from the i18n/ directory.
#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_doesnt_panic() {
        _ = languages().collect::<Vec<_>>();
    }
}
