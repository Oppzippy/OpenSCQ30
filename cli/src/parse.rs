mod primitives;

use std::borrow::Cow;

use anyhow::{anyhow, bail};
use openscq30_lib::settings::{self, ModifiableSelectCommand, Setting, Value};

pub fn setting_value(setting: &Setting, unparsed: Option<String>) -> anyhow::Result<Value> {
    let required_err = anyhow!("a value is required");
    match setting {
        Setting::Toggle { .. } => parse_toggle(&unparsed.ok_or(required_err)?),
        Setting::I32Range { setting, .. } => {
            parse_i32_range(setting, &unparsed.ok_or(required_err)?)
        }
        Setting::Select { setting, .. } => parse_select(setting, &unparsed.ok_or(required_err)?),
        Setting::OptionalSelect { setting, .. } => {
            parse_optional_select(setting, &unparsed.ok_or(required_err)?)
        }
        Setting::ModifiableSelect { setting, .. } => {
            parse_modifiable_select(setting, unparsed.ok_or(required_err)?)
        }
        Setting::MultiSelect { setting, .. } => {
            parse_multi_select(setting, &unparsed.ok_or(required_err)?)
        }
        Setting::Equalizer { setting, .. } => {
            parse_equalizer(setting, &unparsed.ok_or(required_err)?)
        }
        Setting::Information { .. } => parse_information(),
        Setting::ImportString { .. } => parse_import_string(unparsed.ok_or(required_err)?),
        Setting::Action => Ok(Value::Bool(true)),
    }
}

fn parse_toggle(unparsed: &str) -> anyhow::Result<Value> {
    primitives::bool(unparsed).map(Value::from)
}

fn parse_i32_range(setting: &settings::Range<i32>, unparsed: &str) -> anyhow::Result<Value> {
    let number = primitives::i32(unparsed)?;
    if !setting.range.contains(&number) {
        bail!("{number} is out of the expected range {:?}", setting.range)
    }
    if number % setting.step != 0 {
        bail!("{number} does not align with step size {:?}", setting.step)
    }
    Ok(number.into())
}

fn parse_select(setting: &settings::Select, unparsed: &str) -> anyhow::Result<Value> {
    let selection = primitives::one_of_options(unparsed, &setting.options)?;
    Ok(Value::String(selection.clone()))
}

fn parse_optional_select(setting: &settings::Select, unparsed: &str) -> anyhow::Result<Value> {
    if unparsed.is_empty() {
        Ok(Value::OptionalString(None))
    } else {
        let selection = primitives::one_of_options(unparsed, &setting.options)?;
        Ok(Value::OptionalString(Some(selection.clone())))
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
        Ok(primitives::one_of_options(&name, &setting.options)?
            .clone()
            .into())
    }
}

fn parse_multi_select(setting: &settings::Select, unparsed: &str) -> anyhow::Result<Value> {
    primitives::many_of_options(unparsed, &setting.options).map(Value::from)
}

fn parse_equalizer(setting: &settings::Equalizer, unparsed: &str) -> anyhow::Result<Value> {
    let values = primitives::i16_vec(unparsed)?;
    if values.len() != setting.band_hz.len() {
        bail!(
            "wanted {} bands, got {}",
            setting.band_hz.len(),
            values.len()
        );
    }
    for (i, value) in values.iter().copied().enumerate() {
        if value < setting.min || value > setting.max {
            bail!(
                "{} band value {value} is outside of expected range {} to {}",
                // ideally display hz, but fall back to index if not possible
                setting
                    .band_hz
                    .get(i)
                    .map_or_else(|| format!("#{}", i as u16 + 1), |hz| format!("{hz} Hz")),
                setting.min,
                setting.max
            );
        }
    }
    Ok(Value::I16Vec(values))
}

fn parse_information() -> anyhow::Result<Value> {
    Err(anyhow!("can't set value of read only information setting"))
}

fn parse_import_string(unparsed: String) -> anyhow::Result<Value> {
    Ok(Cow::from(unparsed).into())
}
