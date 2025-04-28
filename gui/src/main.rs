use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod add_device;
mod app;
mod device_selection;
mod device_settings;
pub mod equalizer_line;
mod i18n;
mod openscq30_v1_migration;
mod utils;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env()?,
        )
        .pretty()
        .init();
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);
    openscq30_lib::i18n::init(&requested_languages);

    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<app::AppModel>(settings, ())?;

    Ok(())
}
