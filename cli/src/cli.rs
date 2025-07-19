mod completions;
mod device;
mod pair;

use clap::{ArgAction, ArgMatches, Command, arg, value_parser};
use macaddr::MacAddr6;
use openscq30_lib::devices::DeviceModel;

pub fn build() -> Command {
    let mac_address_arg = arg!(-a --"mac-address" <MAC_ADDRESS> "Device's mac address")
        .required(true)
        .value_parser(value_parser!(MacAddr6));
    let device_model_arg = arg!(-m --model <MODEL> "Device model")
        .required(true)
        .value_parser(value_parser!(DeviceModel));
    Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(arg!(--"debug-errors" "Displays additional information with errors for debugging purposes"))
        .arg(arg!(-v --verbose "Enables logging"))
        .subcommand_required(true)
        .subcommand(
            Command::new("paired-devices")
                .subcommand_required(true)
                .subcommand(
                    Command::new("add")
                        .arg(mac_address_arg.to_owned())
                        .arg(device_model_arg.to_owned())
                        .arg(arg!(--"demo" "Enable demo mode for the device")),
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
        .subcommand(
            Command::new("completions")
                .arg(
                    arg!(-s --shell <SHELL> "Target shell to generate completions for")
                        .required(true)
                        .value_parser(value_parser!(clap_complete::Shell))
                )
        )
}

pub async fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand().unwrap() {
        ("paired-devices", matches) => pair::handle(matches).await?,
        ("device", matches) => device::handle(matches).await?,
        ("completions", matches) => completions::handle(matches)?,
        _ => (),
    }
    Ok(())
}
