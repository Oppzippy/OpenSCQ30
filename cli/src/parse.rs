mod primitives;

use std::{borrow::Cow, sync::LazyLock};

use anyhow::{anyhow, bail};
use openscq30_lib::settings::{self, ModifiableSelectCommand, Setting, Value};
use regex::Regex;

pub fn setting_value(setting: &Setting, unparsed: Option<String>) -> anyhow::Result<Value> {
    let required_err = anyhow!("a value is required");
    match setting {
        Setting::Toggle { .. } => parse_toggle(&unparsed.ok_or(required_err)?),
        Setting::I32Range { setting, .. } => {
            parse_i32_range(setting, &unparsed.ok_or(required_err)?)
        }
        Setting::Select { setting, .. } => parse_select(setting, &unparsed.ok_or(required_err)?),
        Setting::OptionalSelect { setting, .. }
        | Setting::PresetEqualizerProfileSelect {
            select: setting, ..
        } => parse_optional_select(setting, &unparsed.ok_or(required_err)?),
        Setting::ModifiableSelect { setting, .. } => {
            parse_modifiable_select(setting, unparsed.ok_or(required_err)?)
        }
        Setting::MultiSelect { setting, .. } => {
            parse_multi_select(setting, &unparsed.ok_or(required_err)?)
        }
        Setting::MultiSelectWithRemove { setting, .. } => {
            parse_multi_select_with_remove(setting, unparsed.ok_or(required_err)?)
        }
        Setting::Equalizer { setting, .. } => {
            parse_equalizer(setting, &unparsed.ok_or(required_err)?)
        }
        Setting::HueColorPicker { .. } => parse_hue(&unparsed.ok_or(required_err)?),
        Setting::Information { .. } => parse_information(),
        Setting::ImportString { .. } => parse_import_string(unparsed.ok_or(required_err)?),
        Setting::Action => Ok(Value::Bool(true)),
        Setting::TimeOfDay { .. } => parse_time(&unparsed.ok_or(required_err)?),
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

fn parse_multi_select_with_remove(
    setting: &settings::Select,
    unparsed: String,
) -> anyhow::Result<Value> {
    if let Some(rest) = unparsed.strip_prefix("-") {
        Ok(Value::MultiSelectWithRemoveCommand(
            settings::MultiSelectWithRemoveCommand::Remove(rest.to_owned().into()),
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

fn parse_hue(unparsed: &str) -> anyhow::Result<Value> {
    let number = primitives::f32(unparsed)?;
    Ok(number.into())
}

fn parse_information() -> anyhow::Result<Value> {
    Err(anyhow!("can't set value of read only information setting"))
}

fn parse_import_string(unparsed: String) -> anyhow::Result<Value> {
    Ok(Cow::from(unparsed).into())
}

fn parse_time(unparsed: &str) -> anyhow::Result<Value> {
    static TIME_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(\d{1,2}):([0-9]{2})(?: ?([AaPp][Mm]))?$").unwrap());
    let captures = TIME_REGEX.captures(unparsed).ok_or_else(|| {
        anyhow!("invalid time {unparsed}, should be in format \"00:00\" or \"12:00 AM\".")
    })?;

    let hour: i32 = captures[1].parse()?;
    let minute: i32 = captures[2].parse()?;
    let maybe_am_pm = captures.get(3);

    anyhow::ensure!(
        minute >= 0 && minute <= 59,
        "minute {minute} must be between 0 and 59"
    );

    if let Some(am_pm) = maybe_am_pm {
        anyhow::ensure!(
            hour >= 1 && hour <= 12,
            "hour {hour} must be between 1 and 12 when using 12 hour time"
        );
        let zero_to_eleven_hour = if hour == 12 { 0 } else { hour };
        let pm_minutes = if am_pm.as_str().eq_ignore_ascii_case("pm") {
            12 * 60
        } else {
            0
        };
        Ok(Value::I32(pm_minutes + zero_to_eleven_hour * 60 + minute))
    } else {
        anyhow::ensure!(
            hour >= 0 && hour <= 23,
            "hour {hour} must be between 0 and 23 when using 24 hour time"
        );
        Ok(Value::I32(hour * 60 + minute))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_time_12_hour_12_am() {
        let time = parse_time("12:00 am")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 0);
    }

    #[test]
    fn parse_time_12_hour_12_pm() {
        let time = parse_time("12:00 pm")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 60 * 12);
    }

    #[test]
    fn parse_time_12_hour_11_59_am() {
        let time = parse_time("11:59 am")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 60 * 11 + 59);
    }

    #[test]
    fn parse_time_12_hour_11_59_pm() {
        let time = parse_time("11:59 pm")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 60 * 23 + 59);
    }

    #[test]
    fn parse_time_24_hour_00_00() {
        let time = parse_time("00:00")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 0);
    }

    #[test]
    fn parse_time_24_hour_23_59() {
        let time = parse_time("23:59")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 60 * 23 + 59);
    }

    #[test]
    fn parse_time_with_no_space_between_time_and_pm() {
        let time = parse_time("12:00pm")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 12 * 60);
    }

    #[test]
    fn parse_time_with_capital_pm() {
        let time = parse_time("12:00PM")
            .unwrap()
            .try_as_i32()
            .expect("parse_time should always return Value::I32");
        assert_eq!(time, 12 * 60);
    }

    #[test]
    fn parse_time_12_hour_hour_less_than_1() {
        let result = parse_time("00:00 am");
        assert!(
            result.is_err(),
            "should error with hour 0 but got {}",
            result.unwrap().try_as_i32().unwrap()
        );
    }

    #[test]
    fn parse_time_12_hour_hour_greater_than_12() {
        let result = parse_time("13:00 am");
        assert!(
            result.is_err(),
            "should error with hour greater than 12 but got {}",
            result.unwrap().try_as_i32().unwrap()
        );
    }

    #[test]
    fn parse_time_24_hour_hour_greater_than_23() {
        let result = parse_time("24:00");
        assert!(
            result.is_err(),
            "should error with hour greater than 23 but got {}",
            result.unwrap().try_as_i32().unwrap()
        );
    }

    #[test]
    fn parse_time_minutes_greater_than_59_should_error() {
        let result = parse_time("00:60");
        assert!(
            result.is_err(),
            "should error with 60 minutes but got {}",
            result.unwrap().try_as_i32().unwrap()
        );
    }
}
