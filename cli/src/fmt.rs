use std::io::IsTerminal;

use openscq30_lib::api::settings::{Setting, Value};
use tabled::Table;

pub struct CustomDisplaySetting(pub Setting);

impl std::fmt::Display for CustomDisplaySetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Setting::Toggle { .. } => write!(f, "toggle"),
            Setting::I32Range { setting, .. } => write!(
                f,
                "integer range (from: {}, to: {}, step: {})",
                setting.range.start(),
                setting.range.end(),
                setting.step,
            ),
            Setting::Select { setting, .. } => write!(f, "select ({:?})", setting.options),
            Setting::OptionalSelect { setting, .. } => {
                write!(f, "optional select ({:?})", setting.options)
            }
            Setting::ModifiableSelect { setting, .. } => {
                write!(f, "modifiable select ({:?})", setting.options)
            }
            Setting::Equalizer { setting, .. } => write!(
                f,
                "equalizer (bands: {:?}, min: {}, max: {}, fractional digits: {})",
                setting.band_hz, setting.min, setting.max, setting.fraction_digits,
            ),
            Setting::Information { .. } => write!(f, "information (read only)"),
        }?;
        Ok(())
    }
}

pub struct DisplayableValue(pub Value);

impl std::fmt::Display for DisplayableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Value::Bool(value) => write!(f, "{value}"),
            Value::U16(value) => write!(f, "{value}"),
            Value::U16Vec(items) => write!(f, "{items:?}"),
            Value::OptionalU16(value) => {
                if let Some(value) = value {
                    write!(f, "{value}")
                } else {
                    Ok(())
                }
            }
            Value::I16Vec(items) => write!(f, "{items:?}"),
            Value::I32(value) => write!(f, "{value}"),
            Value::String(value) => write!(f, "{value}"),
            Value::StringVec(values) => write!(f, "{values:?}"),
            Value::OptionalString(value) => {
                if let Some(value) = value {
                    write!(f, "{value}")
                } else {
                    Ok(())
                }
            }
        }
    }
}

pub fn apply_tabled_settings(table: &mut Table) {
    if std::io::stdout().is_terminal() {
        if let Some((width, _)) = terminal_size::terminal_size() {
            let settings = tabled::settings::Settings::default()
                .with(
                    tabled::settings::Width::wrap(width.0 as usize)
                        .priority(tabled::settings::peaker::Priority::max(true)),
                )
                .with(tabled::settings::style::Style::sharp());
            table.with(settings);
        } else {
            tracing::warn!("could not determine terminal size for table formatting");
        }
    }
}
