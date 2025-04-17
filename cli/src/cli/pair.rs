use clap::ArgMatches;
use macaddr::MacAddr6;
use openscq30_lib::{api::OpenSCQ30Session, devices::DeviceModel, storage::PairedDevice};
use tabled::{Table, Tabled};

use crate::openscq30_session;

pub async fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let session = openscq30_session().await?;
    match matches.subcommand().unwrap() {
        ("add", matches) => handle_add(matches, &session).await?,
        ("remove", matches) => handle_remove(matches, &session).await?,
        ("list", _matches) => handle_list(matches, &session).await?,
        _ => unreachable!(),
    }
    Ok(())
}

async fn handle_add(matches: &ArgMatches, session: &OpenSCQ30Session) -> anyhow::Result<()> {
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
    Ok(())
}

async fn handle_remove(matches: &ArgMatches, session: &OpenSCQ30Session) -> anyhow::Result<()> {
    session
        .unpair(
            matches
                .get_one::<MacAddr6>("mac-address")
                .unwrap()
                .to_owned(),
        )
        .await?;
    println!("Unpaired");
    Ok(())
}

async fn handle_list(_matches: &ArgMatches, session: &OpenSCQ30Session) -> anyhow::Result<()> {
    let mut table = Table::new(
        session
            .paired_devices()
            .await?
            .into_iter()
            .map(PairedDeviceTableItem::from),
    );
    crate::fmt::apply_tabled_settings(&mut table);
    println!("{table}");
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
