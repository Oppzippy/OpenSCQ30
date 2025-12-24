use anyhow::{Context, anyhow};
use clap::ArgMatches;
use indexmap::IndexMap;
use macaddr::MacAddr6;
use openscq30_lib::{
    device::OpenSCQ30Device,
    settings::{self, CategoryId, SettingId},
};
use serde::Serialize;
use strum::VariantArray;
use tabled::{Table, Tabled};

use crate::{
    fmt::{CustomDisplaySetting, DisplayableValue},
    openscq30_session,
};

pub async fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let session = openscq30_session().await?;
    let mac_address = matches
        .get_one::<MacAddr6>("mac-address")
        .unwrap()
        .to_owned();
    let device = session.connect(mac_address).await?;
    match matches.subcommand().unwrap() {
        ("list-settings", matches) => {
            handle_list_settings(matches, device.as_ref()).await?;
        }
        ("setting", matches) => {
            handle_setting(matches, device.as_ref()).await?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

async fn handle_list_settings(
    matches: &ArgMatches,
    device: &dyn OpenSCQ30Device,
) -> anyhow::Result<()> {
    let json = matches.get_flag("json");
    let no_categories = matches.get_flag("no-categories");
    let no_extended_info = matches.get_flag("no-extended-info");

    if json {
        let settings_by_category = device.settings_by_category();
        match (no_categories, no_extended_info) {
            // all information
            (false, false) => {
                let settings = JsonCategory::from_openscq30_lib(settings_by_category);
                println!("{}", serde_json::to_string_pretty(&settings)?);
            }
            // no categories
            (true, false) => {
                let settings = settings_by_category
                    .into_iter()
                    .flat_map(|(_, settings)| {
                        settings
                            .into_iter()
                            .map(|(setting_id, setting)| (setting_id, JsonSetting::from(setting)))
                    })
                    .collect::<IndexMap<_, _>>();
                println!("{}", serde_json::to_string_pretty(&settings)?);
            }
            // no extended info
            (false, true) => {
                let setting_ids_by_category = device
                    .categories()
                    .into_iter()
                    .map(|category_id| JsonCategoryNoExtendedInfo {
                        category_id,
                        setting_ids: device.settings_in_category(&category_id),
                    })
                    .collect::<Vec<_>>();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&setting_ids_by_category)?
                );
            }
            // no categories or extended info
            (true, true) => {
                let setting_ids = device
                    .categories()
                    .into_iter()
                    .flat_map(|category_id| device.settings_in_category(&category_id))
                    .collect::<Vec<_>>();
                println!("{}", serde_json::to_string_pretty(&setting_ids)?);
            }
        }
    } else {
        for category_id in device.categories() {
            if !no_categories {
                println!("-- {category_id} --");
            }
            for setting_id in device.settings_in_category(&category_id) {
                // Settings that are currently unavailable should not be listed. They may be temporarily unavailabe such
                // as when TWS is disconnected, but they could also be never available such as when a device's firmware
                // is too old for a particular feature.
                if let Some(setting) = device.setting(&setting_id) {
                    if no_extended_info {
                        println!("{setting_id}");
                    } else {
                        println!("{setting_id}: {}", CustomDisplaySetting(setting));
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonCategoryNoExtendedInfo {
    category_id: CategoryId,
    setting_ids: Vec<SettingId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonCategory {
    category_id: CategoryId,
    settings: Vec<JsonSettingWithId>,
}

impl JsonCategory {
    fn from_openscq30_lib(
        categories: IndexMap<CategoryId, IndexMap<SettingId, settings::Setting>>,
    ) -> Vec<Self> {
        categories
            .into_iter()
            .map(|(category_id, settings)| {
                let json_settings = settings
                    .into_iter()
                    .map(|(setting_id, setting)| JsonSettingWithId {
                        setting_id,
                        setting: setting.into(),
                    })
                    .collect::<Vec<_>>();
                Self {
                    category_id,
                    settings: json_settings,
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonSettingWithId {
    setting_id: SettingId,
    #[serde(flatten)]
    setting: JsonSetting,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum JsonSetting {
    Toggle,
    I32Range { setting: settings::Range<i32> },
    Select { setting: settings::Select },
    OptionalSelect { setting: settings::Select },
    ModifiableSelect { setting: settings::Select },
    MultiSelect { setting: settings::Select },
    Equalizer { setting: settings::Equalizer },
    Information,
    ImportString,
    Action,
}

impl From<settings::Setting> for JsonSetting {
    fn from(setting: settings::Setting) -> Self {
        match setting {
            settings::Setting::Toggle { .. } => Self::Toggle,
            settings::Setting::I32Range { setting, .. } => Self::I32Range { setting },
            settings::Setting::Select { setting, .. } => Self::Select { setting },
            settings::Setting::OptionalSelect { setting, .. } => Self::OptionalSelect { setting },
            settings::Setting::ModifiableSelect { setting, .. } => {
                Self::ModifiableSelect { setting }
            }
            settings::Setting::MultiSelect { setting, .. } => Self::MultiSelect { setting },
            settings::Setting::Equalizer { setting, .. } => Self::Equalizer { setting },
            settings::Setting::Information { .. } => Self::Information,
            settings::Setting::ImportString { .. } => Self::ImportString,
            settings::Setting::Action => Self::Action,
        }
    }
}

async fn handle_setting(matches: &ArgMatches, device: &dyn OpenSCQ30Device) -> anyhow::Result<()> {
    let json = matches.get_flag("json");

    let commands = collect_commands(matches)?;

    // Whether this fails at any point or not, we still want to print the table, so make sure to
    // do that before returning the error.
    let mut table_items = Vec::new();
    let result = execute_commands(device, commands, &mut table_items).await;

    if json {
        println!("{}", serde_json::to_string_pretty(&table_items)?);
    } else if !table_items.is_empty() {
        let mut table = Table::new(table_items.into_iter().map(SettingIdValueTableItem::from));
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
    Set(SettingId, Option<String>),
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
            let (setting_id, unparsed_value) = setting_id_to_value.split_once("=").map_or(
                (setting_id_to_value.as_str(), None),
                |(setting_id, unparsed_value)| (setting_id, Some(unparsed_value)),
            );
            Ok(ExecCommandWithIndex {
                index,
                command: ExecCommand::Set(
                    setting_id_from_str(setting_id)?,
                    unparsed_value.map(ToString::to_string),
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
    table_items: &mut Vec<SettingIdValue>,
) -> anyhow::Result<()> {
    for command in commands {
        match command {
            ExecCommand::Get(setting_id) => {
                let setting = device.setting(&setting_id).ok_or(anyhow!(
                    "{} does not use setting id {setting_id}.",
                    device.model(),
                ))?;
                table_items.push(SettingIdValue {
                    setting_id,
                    value: setting.into(),
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
                        crate::parse::setting_value(&setting, unparsed_value)
                            .context(setting_id)?,
                    )])
                    .await?;
            }
        }
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SettingIdValue {
    setting_id: SettingId,
    value: settings::Value,
}

#[derive(Tabled)]
struct SettingIdValueTableItem {
    #[tabled(rename = "Setting ID")]
    setting_id: SettingId,
    #[tabled(rename = "Value")]
    value: DisplayableValue,
}

impl From<SettingIdValue> for SettingIdValueTableItem {
    fn from(value: SettingIdValue) -> Self {
        Self {
            setting_id: value.setting_id,
            value: DisplayableValue(value.value),
        }
    }
}
