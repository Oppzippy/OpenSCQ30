use std::str::FromStr;

use i18n_embed::unic_langid::LanguageIdentifier;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod add_device;
mod app;
mod config;
mod device_selection;
mod device_settings;
pub mod equalizer_line;
mod i18n;
pub mod icons;
mod openscq30_v1_migration;
mod throttle;
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
    #[cfg(windows)]
    match is_launched_from_console() {
        Ok(true) => (),
        Ok(false) => {
            if let Err(err) = detach_from_console() {
                tracing::error!("error detaching from console: {err:?}")
            }
        }
        Err(err) => tracing::error!("error checking if we're attached to a console: {err:?}"),
    }

    let config_dir = dirs::config_dir()
        .expect("failed to find config dir")
        .join("openscq30");

    let config = config::Config::new(config_dir.join("openscq30-gui-config.toml")).unwrap();

    let requested_languages = {
        let mut requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
        if let Some(language) = &config.get().preferred_language {
            match LanguageIdentifier::from_str(&language) {
                Ok(language_identifier) => requested_languages.insert(0, language_identifier),
                Err(err) => tracing::warn!(
                    "failed to parse preferred language from config file: {language}, {err:?}"
                ),
            }
        }
        requested_languages
    };

    i18n::init(&requested_languages);
    openscq30_lib::i18n::init(&requested_languages);

    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<app::AppModel>(settings, app::AppFlags { config, config_dir })?;

    Ok(())
}

#[cfg(windows)]
fn is_launched_from_console() -> anyhow::Result<bool> {
    use sysinfo::{ProcessRefreshKind, RefreshKind, System};

    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );
    let parent_process_name = sys
        .process(sysinfo::get_current_pid().map_err(|err| anyhow::anyhow!("{err}"))?)
        .and_then(|process| process.parent())
        .and_then(|parent_pid| sys.process(parent_pid))
        .map(|parent| parent.name())
        .ok_or_else(|| anyhow::anyhow!("failed to get parent process name"))?;
    Ok(!parent_process_name.eq_ignore_ascii_case("explorer.exe"))
}

#[cfg(windows)]
fn detach_from_console() -> anyhow::Result<()> {
    unsafe { windows::Win32::System::Console::FreeConsole().map_err(Into::into) }
}
