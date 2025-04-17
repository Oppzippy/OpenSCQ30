use std::str::FromStr;

use anyhow::{anyhow, bail};
use clap::ArgMatches;
use macaddr::MacAddr6;
use openscq30_lib::api::settings::{Setting, SettingId, Value};
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
            let mac_address = matches
                .get_one::<MacAddr6>("mac-address")
                .unwrap()
                .to_owned();
            let no_headers = matches.get_flag("no-headers");
            let no_extended_info = matches.get_flag("no-extended-info");

            let device = session.connect(mac_address).await?;
            for category_id in device.categories() {
                if !no_headers {
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
        }
        ("exec", matches) => {
            let mac_address = matches
                .get_one::<MacAddr6>("mac-address")
                .unwrap()
                .to_owned();
            let mut get_indices = matches.indices_of("get").unwrap_or_default();
            let mut set_indices = matches.indices_of("set").unwrap_or_default();
            let mut commands: Vec<ExecCommandWithIndex> = Vec::new();
            for setting_id in matches.get_many::<String>("get").unwrap_or_default() {
                let index = get_indices.next().unwrap();
                commands.push(ExecCommandWithIndex {
                    index,
                    command: ExecCommand::Get(
                        *SettingId::VARIANTS
                            .into_iter()
                            .find(|variant| {
                                <&'static str>::from(*variant).eq_ignore_ascii_case(setting_id)
                            })
                            .ok_or_else(|| anyhow!("setting id {setting_id} does not exist"))?,
                    ),
                });
            }
            for setting_id_to_value in matches.get_many::<String>("set").unwrap_or_default() {
                let index = set_indices.next().unwrap();
                let (setting_id, unparsed_value) = setting_id_to_value.split_once("=").ok_or(
                    anyhow!("invalid setting id value assignment: {setting_id_to_value}"),
                )?;
                commands.push(ExecCommandWithIndex {
                    index,
                    command: ExecCommand::Set(
                        *SettingId::VARIANTS
                            .into_iter()
                            .find(|variant| {
                                <&'static str>::from(*variant).eq_ignore_ascii_case(setting_id)
                            })
                            .ok_or_else(|| anyhow!("setting id {setting_id} does not exist"))?,
                        unparsed_value.to_string(),
                    ),
                });
            }
            commands.sort();

            let mut table_items = Vec::new();
            let device = session.connect(mac_address).await?;

            // Whether this fails at any point or not, we still want to print the table, so make sure to
            // do that before returning the error.
            let result = async {
                for command in commands {
                    let command = command.command;
                    match command {
                        ExecCommand::Get(setting_id) => {
                            if let Some(setting) = device.setting(&setting_id) {
                                table_items.push(SettingIdValueTableItem {
                                    setting_id,
                                    value: DisplayableValue(setting.into()),
                                });
                            }
                        }
                        ExecCommand::Set(setting_id, unparsed_value) => {
                            let setting = device.setting(&setting_id).ok_or(anyhow!(
                                "the current device does not use setting id {setting_id}."
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
                Ok(()) as anyhow::Result<()>
            }
            .await;

            if !table_items.is_empty() {
                let mut table = Table::new(&table_items);
                crate::fmt::apply_tabled_settings(&mut table);
                println!("{table}");
            } else if result.is_ok() {
                println!("OK");
            }

            result?;
        }
        _ => unreachable!(),
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
            Value::OptionalString((!unparsed_value.is_empty()).then_some(unparsed_value.into()))
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
    };
    Ok(value)
}
