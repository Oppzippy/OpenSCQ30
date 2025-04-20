use tracing::Level;

mod add_device;
mod app;
mod device_selection;
mod device_settings;
pub mod equalizer_line;
mod i18n;
mod openscq30_v1_migration;
mod utils;

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_max_level(Level::WARN)
        .pretty()
        .init();
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);
    openscq30_lib::i18n::init(&requested_languages);

    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<app::AppModel>(settings, ())
}
