use anyhow::{Context, anyhow};
use clap::ArgMatches;
use macaddr::MacAddr6;
use openscq30_lib::api::{OpenSCQ30Session, device::OpenSCQ30Device, settings::SettingId};
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
                println!("{setting_id}: {}", CustomDisplaySetting(setting));
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
                        crate::parse::setting_value(&setting, unparsed_value)
                            .context(setting_id)?,
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
