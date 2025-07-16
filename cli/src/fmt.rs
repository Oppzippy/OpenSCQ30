use std::io::IsTerminal;

use openscq30_lib::api::settings::{Setting, Value};
use strum::Display;
use tabled::{
    Table,
    settings::{Alignment, Padding, Settings, Style, Width, peaker::Priority},
};

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
            Setting::MultiSelect { setting, .. } => {
                write!(f, "multi select ({:?})", setting.options)
            }
            Setting::Equalizer { setting, .. } => write!(
                f,
                "equalizer (bands: {:?}, min: {}, max: {}, fractional digits: {})",
                setting.band_hz, setting.min, setting.max, setting.fraction_digits,
            ),
            Setting::Information { .. } => write!(f, "information (read only)"),
            Setting::ImportString { .. } => write!(f, "import string"),
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
            Value::StringVec(values) => {
                let mut buffer = Vec::new();
                let mut writer = csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(&mut buffer);
                // write the fields individually rather than write_record so that we don't get a newline at the end
                for value in values {
                    writer.write_field(value.as_bytes()).unwrap();
                }
                std::mem::drop(writer);
                write!(f, "{}", String::from_utf8(buffer).unwrap())
            }
            Value::OptionalString(value) => {
                if let Some(value) = value {
                    write!(f, "{value}")
                } else {
                    Ok(())
                }
            }
            Value::ModifiableSelectCommand(_) => {
                unimplemented!("this should not be shown to the user")
            }
        }
    }
}

pub fn apply_tabled_settings(table: &mut Table) {
    if std::io::stdout().is_terminal() {
        if let Some((width, _)) = terminal_size::terminal_size() {
            let settings = Settings::default()
                .with(Width::wrap(width.0 as usize).priority(Priority::max(true)))
                .with(Style::sharp());
            table.with(settings);
        } else {
            tracing::warn!("could not determine terminal size for table formatting");
        }
    } else {
        table
            .with(Style::empty().vertical('\t'))
            .with(Alignment::left())
            .with(Padding::zero());
    }
}

#[derive(Debug, Display)]
pub enum YesOrNo {
    Yes,
    No,
}

impl From<bool> for YesOrNo {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Yes,
            false => Self::No,
        }
    }
}
