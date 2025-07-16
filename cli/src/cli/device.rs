use std::{borrow::Cow, str::FromStr};

use anyhow::{anyhow, bail};
use clap::ArgMatches;
use macaddr::MacAddr6;
use openscq30_lib::api::{
    OpenSCQ30Session,
    device::OpenSCQ30Device,
    settings::{ModifiableSelectCommand, Setting, SettingId, Value},
};
use strum::VariantArray;
use tabled::{Table, Tabled};

use crate::{
    fmt::{CustomDisplaySetting, DisplayableValue},
    openscq30_session,
};

pub async fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let session = openscq30_session().await?;
    match matches.subcommand().unwrap() {
        ("list-settings", matches) => {
            handle_list_settings(matches, &session).await?;
        }
        ("exec", matches) => {
            handle_exec(matches, &session).await?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

async fn handle_list_settings(
    matches: &ArgMatches,
    session: &OpenSCQ30Session,
) -> anyhow::Result<()> {
    let mac_address = matches
        .get_one::<MacAddr6>("mac-address")
        .unwrap()
        .to_owned();
    let no_categories = matches.get_flag("no-categories");
    let no_extended_info = matches.get_flag("no-extended-info");

    let device = session.connect(mac_address).await?;
    for category_id in device.categories() {
        if !no_categories {
            println!("-- {category_id} --");
        }
        for setting_id in device.settings_in_category(&category_id) {
            if no_extended_info {
                println!("{setting_id}");
            } else {
                let setting = device.setting(&setting_id).unwrap();
                println!("{setting_id}: {}", CustomDisplaySetting(setting))
            }
        }
    }

    Ok(())
}

async fn handle_exec(matches: &ArgMatches, session: &OpenSCQ30Session) -> anyhow::Result<()> {
    let mac_address = matches
        .get_one::<MacAddr6>("mac-address")
        .unwrap()
        .to_owned();
    let commands = collect_commands(matches)?;

    let device = session.connect(mac_address).await?;

    // Whether this fails at any point or not, we still want to print the table, so make sure to
    // do that before returning the error.
    let mut table_items = Vec::new();
    let result = execute_commands(device.as_ref(), commands, &mut table_items).await;

    if !table_items.is_empty() {
        let mut table = Table::new(&table_items);
        crate::fmt::apply_tabled_settings(&mut table);
        println!("{table}");
    } else if result.is_ok() {
        println!("OK");
    }

    result
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ExecCommandWithIndex {
    index: usize,
    command: ExecCommand,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ExecCommand {
    Get(SettingId),
    Set(SettingId, String),
}

fn collect_commands(matches: &ArgMatches) -> anyhow::Result<Vec<ExecCommand>> {
    let get_iter = matches
        .indices_of("get")
        .unwrap_or_default()
        .zip(matches.get_many::<String>("get").unwrap_or_default())
        .map(|(index, setting_id)| {
            Ok(ExecCommandWithIndex {
                index,
                command: ExecCommand::Get(setting_id_from_str(setting_id)?),
            })
        });

    let set_iter = matches
        .indices_of("set")
        .unwrap_or_default()
        .zip(matches.get_many::<String>("set").unwrap_or_default())
        .map(|(index, setting_id_to_value)| {
            let (setting_id, unparsed_value) = setting_id_to_value.split_once("=").ok_or(
                anyhow!("invalid setting id value assignment: {setting_id_to_value}"),
            )?;
            Ok(ExecCommandWithIndex {
                index,
                command: ExecCommand::Set(
                    setting_id_from_str(setting_id)?,
                    unparsed_value.to_string(),
                ),
            })
        });

    let mut commands = get_iter
        .chain(set_iter)
        .collect::<anyhow::Result<Vec<_>>>()?;
    commands.sort();
    Ok(commands
        .into_iter()
        .map(|with_index| with_index.command)
        .collect())
}

fn setting_id_from_str(setting_id: &str) -> anyhow::Result<SettingId> {
    Ok(*SettingId::VARIANTS
        .iter()
        .find(|variant| <&'static str>::from(*variant).eq_ignore_ascii_case(setting_id))
        .ok_or_else(|| anyhow!("setting id {setting_id} does not exist"))?)
}

async fn execute_commands(
    device: &dyn OpenSCQ30Device,
    commands: Vec<ExecCommand>,
    table_items: &mut Vec<SettingIdValueTableItem>,
) -> anyhow::Result<()> {
    for command in commands {
        match command {
            ExecCommand::Get(setting_id) => {
                let setting = device.setting(&setting_id).ok_or(anyhow!(
                    "{} does not use setting id {setting_id}.",
                    device.model(),
                ))?;
                table_items.push(SettingIdValueTableItem {
                    setting_id,
                    value: DisplayableValue(setting.into()),
                });
            }
            ExecCommand::Set(setting_id, unparsed_value) => {
                let setting = device.setting(&setting_id).ok_or(anyhow!(
                    "{} does not use setting id {setting_id}.",
                    device.model(),
                ))?;
                device
                    .set_setting_values(vec![(
                        setting_id,
                        parse_setting_value(&setting, unparsed_value)?,
                    )])
                    .await?;
            }
        }
    }
    Ok(())
}

#[derive(Tabled)]
struct SettingIdValueTableItem {
    #[tabled(rename = "Setting ID")]
    setting_id: SettingId,
    #[tabled(rename = "Value")]
    value: DisplayableValue,
}

fn parse_setting_value(setting: &Setting, unparsed_value: String) -> anyhow::Result<Value> {
    let value: Value = match setting {
        Setting::Toggle { .. } => bool::from_str(&unparsed_value)?.into(),
        Setting::I32Range { setting, .. } => {
            let value = i32::from_str(&unparsed_value)?;
            if !setting.range.contains(&value) {
                bail!("{value} is out of the expected range {:?}", setting.range)
            }
            value.into()
        }
        Setting::Select { setting, .. } => {
            let value = setting
                .options
                .iter()
                .find(|option| option.eq_ignore_ascii_case(&unparsed_value))
                .ok_or_else(|| {
                    anyhow!(
                        "{unparsed_value} is not a valid option. Expected one of: {:?}",
                        setting.options
                    )
                })?;
            Value::String(value.clone())
        }
        Setting::OptionalSelect { setting, .. } => {
            let value = setting
                .options
                .iter()
                .find(|option| option.eq_ignore_ascii_case(&unparsed_value));
            Value::OptionalString(value.cloned())
        }
        Setting::ModifiableSelect { setting: _, .. } => {
            if let Some(rest) = unparsed_value.strip_prefix("+") {
                Value::ModifiableSelectCommand(ModifiableSelectCommand::Add(rest.to_owned().into()))
            } else if let Some(rest) = unparsed_value.strip_prefix("-") {
                Value::ModifiableSelectCommand(ModifiableSelectCommand::Remove(
                    rest.to_owned().into(),
                ))
            } else {
                // To allow selecting profiles that start with a '+' or '-' without triggering the other
                // branches, '\' can be used as a prefix that will be ignored.
                let name = unparsed_value
                    .strip_prefix("\\")
                    .map(ToOwned::to_owned)
                    .unwrap_or(unparsed_value);
                Value::String(name.into())
            }
        }
        Setting::MultiSelect { setting: _, .. } => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(unparsed_value.as_bytes());
            let maybe_row = reader.records().next().transpose()?;
            let strings = maybe_row
                .map(|row| {
                    row.into_iter()
                        .map(|entry| Cow::from(entry.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            Value::StringVec(strings)
        }
        Setting::Equalizer { setting, .. } => {
            let values = unparsed_value
                .split(",")
                .enumerate()
                .map(|(i, unparsed)| {
                    let value = i16::from_str(unparsed).map_err(anyhow::Error::from)?;
                    if value < setting.min || value > setting.max {
                        bail!(
                            "{} band value {value} is outside of expected range {} to {}",
                            // ideally display hz, but fall back to index if not possible
                            setting.band_hz.get(i).cloned().unwrap_or(i as u16 + 1),
                            setting.min,
                            setting.max
                        );
                    }
                    Ok(value)
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            Value::I16Vec(values)
        }
        Setting::Information { .. } => bail!("can't set value of read only information setting"),
        Setting::ImportString { .. } => Cow::from(unparsed_value).into(),
    };
    Ok(value)
}
