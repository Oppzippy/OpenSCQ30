use std::{error::Error, sync::Arc};

use clap::Parser;
use cli::{Cli, Command};
use openscq30_lib::api::device::{DeviceDescriptor, DeviceRegistry};

mod cli;
mod get;
mod list_devices;
mod set;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(args.logging_level)
        .with_writer(std::io::stderr)
        .pretty()
        .init();

    let registry = openscq30_lib::api::new_soundcore_device_registry()
        .await
        .unwrap_or_else(|err| panic!("failed to initialize device registry: {err}"));
    do_cli_command(args, registry).await
}

// rust-analyzer doesn't seem to work with the associated types of an impl Trait return value
// as a workaround, we can immediately pass the return value as a parameter to another function
async fn do_cli_command(args: Cli, registry: impl DeviceRegistry) -> Result<(), Box<dyn Error>> {
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
        (_, None) => eprintln!("No device found."),
    };
    Ok(())
}

async fn get_device_or_err<T>(
    registry: &T,
    descriptor: &T::DescriptorType,
) -> Result<Arc<T::DeviceType>, String>
where
    T: DeviceRegistry,
{
    match registry.device(descriptor.mac_address()).await {
        Ok(Some(device)) => Ok(device),
        Err(err) => Err(format!("Error fetching device: {err}")),
        Ok(None) => Err("No device found.".to_string()),
    }
}
