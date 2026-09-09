use clap::ArgMatches;
use macaddr::MacAddr6;
use openscq30_lib::{DeviceModel, OpenSCQ30Session, storage::PairedDevice};
use serde::Serialize;
use tabled::{Table, Tabled};

use crate::{fmt::YesOrNo, openscq30_session};

pub async fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let session = openscq30_session().await?;
    match matches.subcommand().unwrap() {
        ("add", matches) => handle_add(matches, &session).await?,
        ("remove", matches) => handle_remove(matches, &session).await?,
        ("list", matches) => handle_list(matches, &session).await?,
        _ => unreachable!(),
    }
    Ok(())
}

async fn handle_add(matches: &ArgMatches, session: &OpenSCQ30Session) -> anyhow::Result<()> {
    let paired_device = PairedDevice {
        mac_address: matches
            .get_one::<MacAddr6>("mac-address")
            .unwrap()
            .to_owned(),
        model: matches.get_one::<DeviceModel>("model").unwrap().to_owned(),
        is_demo: matches.get_flag("demo"),
    };
    session.pair(paired_device).await?;
    if matches.get_flag("json") {
        println!("{}", serde_json::to_string_pretty(&paired_device)?);
    } else {
        println!("Paired");
    }
    Ok(())
}

async fn handle_remove(matches: &ArgMatches, session: &OpenSCQ30Session) -> anyhow::Result<()> {
    let mac_address = matches
        .get_one::<MacAddr6>("mac-address")
        .unwrap()
        .to_owned();
    session.unpair(mac_address).await?;
    if matches.get_flag("json") {
        println!(
            "{}",
            serde_json::to_string_pretty(&JsonRemoveResult {
                mac_address: mac_address.to_string(),
            })?
        );
    } else {
        println!("Unpaired");
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRemoveResult {
    mac_address: String,
}

async fn handle_list(matches: &ArgMatches, session: &OpenSCQ30Session) -> anyhow::Result<()> {
    let paired_devices = session.paired_devices().await?;
    if matches.get_flag("json") {
        let json = serde_json::to_string_pretty(&paired_devices)?;
        println!("{json}");
    } else {
        let mut table = Table::new(paired_devices.into_iter().map(PairedDeviceTableItem::from));
        crate::fmt::apply_tabled_settings(&mut table);
        println!("{table}");
    }
    Ok(())
}

#[derive(Tabled)]
struct PairedDeviceTableItem {
    #[tabled(rename = "Device Model")]
    model: DeviceModel,
    #[tabled(rename = "MAC Address")]
    mac_address: MacAddr6,
    #[tabled(rename = "Demo Mode")]
    demo_mode: YesOrNo,
}

impl From<PairedDevice> for PairedDeviceTableItem {
    fn from(value: PairedDevice) -> Self {
        Self {
            mac_address: value.mac_address,
            model: value.model,
            demo_mode: value.is_demo.into(),
        }
    }
}
