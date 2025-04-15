use anyhow::{anyhow, bail};
use dirs::config_dir;
use std::{fmt, io::IsTerminal, str::FromStr};
use strum::VariantArray;
use tabled::{
    Table, Tabled,
    settings::{Width, peaker::Priority},
};
use terminal_size::terminal_size;

use clap::{ArgAction, ArgMatches, Command, arg, value_parser};
use macaddr::MacAddr6;
use openscq30_lib::{
    api::{
        OpenSCQ30Session,
        settings::{Setting, SettingId, Value},
    },
    devices::DeviceModel,
    storage::PairedDevice,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mac_address_arg = arg!(-a --"mac-address" <MAC_ADDRESS> "Device's mac address")
        .required(true)
        .value_parser(value_parser!(MacAddr6));
    let matches = Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(
            Command::new("paired-devices")
                .subcommand_required(true)
                .subcommand(
                    Command::new("add")
                        .arg(arg!(-n --name <NAME> "Display name for the device. Does not have to be unique.").required(true))
                        .arg(mac_address_arg.to_owned())
                        .arg(arg!(-m --model <MODEL> "Device model").required(true)),
                )
                .subcommand(
                    Command::new("remove").alias("delete")
                        .arg(mac_address_arg.to_owned())
                )
                .subcommand(Command::new("list").alias("ls")),
        )
        .subcommand(
            Command::new("device")
                .subcommand_required(true)
                .subcommand(
                    Command::new("list-settings")
                        .arg(mac_address_arg.to_owned())
                        .arg(arg!(--"no-categories" "Don't display category headers"))
                        .arg(arg!(--"no-extended-info" "Don't display setting information in addition to the setting id")),
                )
                .subcommand(
                    Command::new("exec")
                        .arg(mac_address_arg.to_owned())
                        .arg(
                            arg!(-g --get <SETTING_ID> "Gets the value of a setting")
                                .action(ArgAction::Append),
                        )
                        .arg(
                            arg!(-s --set <"SETTING_ID=VALUE"> "Sets the value of a setting.")
                                .action(ArgAction::Append),
                        ),
                )
        )
        .get_matches();

    match matches.subcommand().unwrap() {
        ("paired-devices", matches) => handle_paired_devices(matches).await,
        ("device", matches) => handle_device(matches).await,
        _ => Ok(()),
    }
}

async fn openscq30_session() -> anyhow::Result<OpenSCQ30Session> {
    OpenSCQ30Session::new(
        config_dir()
            .expect("failed to find config dir")
            .join("openscq30")
            .join("database.sqlite"),
    )
    .await
    .map_err(Into::into)
}

async fn handle_device(matches: &ArgMatches) -> anyhow::Result<()> {
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
                apply_tabled_settings(&mut table);
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

async fn handle_paired_devices(matches: &ArgMatches) -> anyhow::Result<()> {
    let session = openscq30_session().await?;
    match matches.subcommand().unwrap() {
        ("add", matches) => {
            session
                .pair(PairedDevice {
                    name: matches.get_one::<String>("name").unwrap().to_owned(),
                    mac_address: matches
                        .get_one::<MacAddr6>("mac-address")
                        .unwrap()
                        .to_owned(),
                    model: matches.get_one::<DeviceModel>("model").unwrap().to_owned(),
                })
                .await?;
            println!("Paired");
        }
        ("remove", matches) => {
            session
                .unpair(
                    matches
                        .get_one::<MacAddr6>("mac-address")
                        .unwrap()
                        .to_owned(),
                )
                .await?;
            println!("Unpaired");
        }
        ("list", _matches) => {
            let mut table = Table::new(
                session
                    .paired_devices()
                    .await?
                    .into_iter()
                    .map(PairedDeviceTableItem::from),
            );
            apply_tabled_settings(&mut table);
            println!("{table}");
        }
        _ => unreachable!(),
    }
    Ok(())
}

#[derive(Tabled)]
struct PairedDeviceTableItem {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "MAC Address")]
    mac_address: MacAddr6,
    #[tabled(rename = "Device Model")]
    model: DeviceModel,
}

impl From<PairedDevice> for PairedDeviceTableItem {
    fn from(value: PairedDevice) -> Self {
        Self {
            name: value.name,
            mac_address: value.mac_address,
            model: value.model,
        }
    }
}

struct CustomDisplaySetting(Setting);
impl fmt::Display for CustomDisplaySetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

struct DisplayableValue(Value);

impl fmt::Display for DisplayableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

fn apply_tabled_settings(table: &mut Table) {
    if std::io::stdout().is_terminal() {
        let term_width = terminal_size().unwrap().0.0 as usize;
        table.with(
            tabled::settings::Settings::default()
                .with(Width::wrap(term_width).priority(Priority::max(true)))
                .with(tabled::settings::style::Style::sharp()),
        );
    }
}
