use std::{error::Error, rc::Rc};

use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use cli::{Cli, Command};
use openscq30_lib::api::device::{DeviceDescriptor, DeviceRegistry};

mod cli;
mod get;
mod list_devices;
mod set;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut command = Cli::command();
    let command_name = command.get_name().to_string();
    if let Command::Completions { shell } = args.command {
        clap_complete::generate(
            Shell::from(shell),
            &mut command,
            command_name,
            &mut std::io::stdout(),
        );
        return Ok(());
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        tracing_subscriber::fmt()
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_max_level(args.logging_level)
            .with_writer(std::io::stderr)
            .pretty()
            .init();

        let registry =
            openscq30_lib::api::new_soundcore_device_registry(runtime.handle().to_owned())
                .await
                .unwrap_or_else(|err| panic!("failed to initialize device registry: {err}"));

        let descriptors = registry.device_descriptors().await?;
        let selected_descriptor = args
            .mac_address
            .map(|mac_address| {
                descriptors
                    .iter()
                    .find(|descriptor| descriptor.mac_address() == mac_address)
            })
            .or_else(|| Some(descriptors.first()))
            .flatten();

        match (args.command, selected_descriptor) {
            (Command::ListDevices, _) => list_devices::list_devices(&descriptors),
            (Command::Set(set_command), Some(descriptor)) => {
                let device = get_device_or_err(&registry, descriptor).await?;
                set::set(set_command, device.as_ref()).await?;
            }
            (Command::Get(get_command), Some(descriptor)) => {
                let device = get_device_or_err(&registry, descriptor).await?;
                get::get(get_command, device.as_ref()).await;
            }
            (Command::Completions { .. }, Some(_)) => unreachable!(),
            (_, None) => eprintln!("No device found."),
        };
        Ok(())
    })
}

async fn get_device_or_err<T>(
    registry: &T,
    descriptor: &T::DescriptorType,
) -> Result<Rc<T::DeviceType>, String>
where
    T: DeviceRegistry,
{
    match registry.device(descriptor.mac_address()).await {
        Ok(Some(device)) => Ok(device),
        Err(err) => Err(format!("Error fetching device: {err}")),
        Ok(None) => Err("No device found.".to_string()),
    }
}
