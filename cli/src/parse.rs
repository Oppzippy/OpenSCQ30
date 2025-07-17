use std::{borrow::Cow, collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail};
use openscq30_lib::api::settings::{self, ModifiableSelectCommand, Setting, Value};

pub fn setting_value(setting: &Setting, unparsed: String) -> anyhow::Result<Value> {
    match setting {
        Setting::Toggle { .. } => parse_toggle(&unparsed),
        Setting::I32Range { setting, .. } => parse_i32_range(setting, &unparsed),
        Setting::Select { setting, .. } => parse_select(setting, &unparsed),
        Setting::OptionalSelect { setting, .. } => parse_optional_select(setting, &unparsed),
        Setting::ModifiableSelect { setting, .. } => parse_modifiable_select(setting, unparsed),
        Setting::MultiSelect { setting, .. } => parse_multi_select(setting, &unparsed),
        Setting::Equalizer { setting, .. } => parse_equalizer(setting, &unparsed),
        Setting::Information { .. } => parse_information(),
        Setting::ImportString { .. } => parse_import_string(unparsed),
    }
}

fn parse_toggle(unparsed: &str) -> anyhow::Result<Value> {
    Ok(bool::from_str(&unparsed)?.into())
}

fn parse_i32_range(setting: &settings::Range<i32>, unparsed: &str) -> anyhow::Result<Value> {
    let value = i32::from_str(unparsed)?;
    if !setting.range.contains(&value) {
        bail!("{value} is out of the expected range {:?}", setting.range)
    }
    Ok(value.into())
}

fn parse_select(setting: &settings::Select, unparsed: &str) -> anyhow::Result<Value> {
    let value = setting
        .options
        .iter()
        .find(|option| option.eq_ignore_ascii_case(&unparsed))
        .ok_or_else(|| {
            anyhow!(
                "{unparsed} is not a valid option. Expected one of: {:?}",
                setting.options
            )
        })?;
    Ok(Value::String(value.clone()))
}

fn parse_optional_select(setting: &settings::Select, unparsed: &str) -> anyhow::Result<Value> {
    if unparsed.is_empty() {
        Ok(Value::OptionalString(None))
    } else {
        let value = setting
                .options
                .iter()
                .find(|option| option.eq_ignore_ascii_case(&unparsed))
                .ok_or_else(|| {
                    anyhow!(
                        "{unparsed} is not a valid option. Expected either an empty string or one of: {:?}",
                        setting.options
                    )
                })?;
        Ok(Value::OptionalString(Some(value.to_owned())))
    }
}

fn parse_modifiable_select(setting: &settings::Select, unparsed: String) -> anyhow::Result<Value> {
    if let Some(rest) = unparsed.strip_prefix("+") {
        Ok(Value::ModifiableSelectCommand(
            ModifiableSelectCommand::Add(rest.to_owned().into()),
        ))
    } else if let Some(rest) = unparsed.strip_prefix("-") {
        Ok(Value::ModifiableSelectCommand(
            ModifiableSelectCommand::Remove(rest.to_owned().into()),
        ))
    } else {
        // To allow selecting profiles that start with a '+' or '-' without triggering the other
        // branches, '\' can be used as a prefix that will be ignored.
        let name = unparsed
            .strip_prefix("\\")
            .map(ToOwned::to_owned)
            .unwrap_or(unparsed);
        if !setting.options.contains(&Cow::Borrowed(&name)) {
            bail!(
                "{name} is not a valid option. Expected one of: {:?}",
                setting.options
            );
        }
        Ok(Value::String(name.into()))
    }
}

fn parse_multi_select(setting: &settings::Select, unparsed: &str) -> anyhow::Result<Value> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(unparsed.as_bytes());
    let maybe_row = reader.records().next().transpose()?;
    let strings = maybe_row
        .map(|row| {
            row.into_iter()
                .map(|entry| entry.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let valid_options_lowercase = setting
        .options
        .iter()
        .map(|option| (option.to_lowercase(), option))
        .collect::<HashMap<_, _>>();
    let options = strings
        .iter()
        .map(|string| {
            valid_options_lowercase
                .get(&string.to_lowercase())
                .cloned()
                .cloned()
                .ok_or_else(|| {
                    anyhow!(
                        "{string} is not a valid option. Expected one of: {:?}",
                        setting.options
                    )
                })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(Value::StringVec(options))
}

fn parse_equalizer(setting: &settings::Equalizer, unparsed: &str) -> anyhow::Result<Value> {
    let values = unparsed
        .split(",")
        .enumerate()
        .map(|(i, unparsed)| {
            let value = i16::from_str(unparsed).map_err(anyhow::Error::from)?;
            if value < setting.min || value > setting.max {
                bail!(
                    "{} band value {value} is outside of expected range {} to {}",
                    // ideally display hz, but fall back to index if not possible
                    setting
                        .band_hz
                        .get(i)
                        .map(|hz| format!("{hz} Hz"))
                        .unwrap_or_else(|| format!("#{}", i as u16 + 1)),
                    setting.min,
                    setting.max
                );
            }
            Ok(value)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    if values.len() != setting.band_hz.len() {
        bail!(
            "wanted {} bands, got {}",
            setting.band_hz.len(),
            values.len()
        )
    }
    Ok(Value::I16Vec(values))
}

fn parse_information() -> anyhow::Result<Value> {
    Err(anyhow!("can't set value of read only information setting"))
}

fn parse_import_string(unparsed: String) -> anyhow::Result<Value> {
    Ok(Cow::from(unparsed).into())
}
