use clap::ArgMatches;
use macaddr::MacAddr6;
use openscq30_lib::{devices::DeviceModel, storage::PairedDevice};
use tabled::{Table, Tabled};

use crate::openscq30_session;

pub async fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
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
            crate::fmt::apply_tabled_settings(&mut table);
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
