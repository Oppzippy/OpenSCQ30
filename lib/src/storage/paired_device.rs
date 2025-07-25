use macaddr::MacAddr6;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};

use crate::devices::DeviceModel;

use super::{
    Error,
    type_conversions::{SqliteDeviceModel, SqliteMacAddr6},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairedDevice {
    #[serde(with = "crate::serialization::mac_addr")]
    pub mac_address: MacAddr6,
    pub model: DeviceModel,
    pub is_demo: bool,
}

#[instrument(skip(connection))]
pub fn fetch_all(connection: &Connection) -> Result<Vec<PairedDevice>, Error> {
    let mut statement = connection.prepare_cached(
        "SELECT mac_address, model, is_demo FROM paired_device ORDER BY model ASC",
    )?;
    let devices = statement
        .query(())?
        .mapped(|row| {
            let mac_address: SqliteMacAddr6 = row.get("mac_address")?;
            let model: SqliteDeviceModel = row.get("model")?;
            let is_demo: bool = row.get("is_demo")?;
            Ok(PairedDevice {
                mac_address: mac_address.0,
                model: model.0,
                is_demo,
            })
        })
        .filter_map(|result| match result {
            Ok(device) => Some(device),
            Err(err) => {
                warn!("error parsing row: {err:?}");
                None
            }
        })
        .collect();
    Ok(devices)
}

#[instrument(skip(connection))]
pub fn fetch(
    connection: &Connection,
    mac_address: MacAddr6,
) -> Result<Option<PairedDevice>, Error> {
    let mut statement = connection.prepare_cached(
        "SELECT mac_address, model, is_demo FROM paired_device WHERE mac_address = ?1",
    )?;
    let devices = statement
        .query([SqliteMacAddr6(mac_address)])?
        .mapped(|row| {
            let mac_address: SqliteMacAddr6 = row.get("mac_address")?;
            let model: SqliteDeviceModel = row.get("model")?;
            let is_demo: bool = row.get("is_demo")?;
            Ok(PairedDevice {
                mac_address: mac_address.0,
                model: model.0,
                is_demo,
            })
        })
        .find_map(|result| match result {
            Ok(device) => Some(device),
            Err(err) => {
                warn!("error parsing row: {err:?}");
                None
            }
        });
    Ok(devices)
}

pub fn insert(connection: &Connection, paired_device: PairedDevice) -> Result<(), Error> {
    connection.execute(
        "INSERT INTO paired_device (mac_address, model, is_demo) VALUES (?1, ?2, ?3)",
        (
            SqliteMacAddr6(paired_device.mac_address),
            SqliteDeviceModel(paired_device.model),
            paired_device.is_demo,
        ),
    )?;
    Ok(())
}

pub fn upsert(connection: &Connection, paired_device: PairedDevice) -> Result<(), Error> {
    connection.execute(
        r#"INSERT INTO paired_device (mac_address, model, is_demo)
                    VALUES (?1, ?2, ?3)
                ON CONFLICT(mac_address) DO UPDATE SET
                    model = excluded.model,
                    is_demo = excluded.is_demo,
                    created_at = strftime('%s')"#,
        (
            SqliteMacAddr6(paired_device.mac_address),
            SqliteDeviceModel(paired_device.model),
            paired_device.is_demo,
        ),
    )?;
    Ok(())
}

pub fn delete(connection: &Connection, mac_address: MacAddr6) -> Result<(), Error> {
    connection.execute(
        "DELETE FROM paired_device WHERE mac_address = ?1",
        (SqliteMacAddr6(mac_address),),
    )?;
    Ok(())
}
