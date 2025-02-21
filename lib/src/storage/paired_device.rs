use macaddr::MacAddr6;
use rusqlite::Connection;
use tracing::{instrument, warn};

use super::{
    type_conversions::{SqliteDeviceModel, SqliteMacAddr6},
    PairedDevice, StorageError,
};

#[instrument(skip(connection))]
pub fn fetch_all(connection: &Connection) -> Result<Vec<PairedDevice>, StorageError> {
    let mut statement = connection
        .prepare_cached("SELECT name, mac_address, model FROM paired_device ORDER BY name ASC")?;
    let devices = statement
        .query(())?
        .mapped(|row| {
            let name: String = row.get("name")?;
            let mac_address: SqliteMacAddr6 = row.get("mac_address")?;
            let model: SqliteDeviceModel = row.get("model")?;
            Ok(PairedDevice {
                name,
                mac_address: mac_address.0,
                model: model.0,
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
) -> Result<Option<PairedDevice>, StorageError> {
    let mut statement = connection.prepare_cached(
        "SELECT name, mac_address, model FROM paired_device WHERE mac_address = ?1",
    )?;
    let devices = statement
        .query([SqliteMacAddr6(mac_address)])?
        .mapped(|row| {
            let name: String = row.get("name")?;
            let mac_address: SqliteMacAddr6 = row.get("mac_address")?;
            let model: SqliteDeviceModel = row.get("model")?;
            Ok(PairedDevice {
                name,
                mac_address: mac_address.0,
                model: model.0,
            })
        })
        .filter_map(|result| match result {
            Ok(device) => Some(device),
            Err(err) => {
                warn!("error parsing row: {err:?}");
                None
            }
        })
        .next();
    Ok(devices)
}

pub fn insert(connection: &Connection, paired_device: PairedDevice) -> Result<(), StorageError> {
    connection.execute(
        "INSERT INTO paired_device (name, mac_address, model) VALUES (?1, ?2, ?3)",
        (
            &paired_device.name,
            SqliteMacAddr6(paired_device.mac_address),
            SqliteDeviceModel(paired_device.model),
        ),
    )?;
    Ok(())
}

pub fn upsert(connection: &Connection, paired_device: PairedDevice) -> Result<(), StorageError> {
    connection.execute(
        r#"INSERT INTO paired_device (name, mac_address, model)
                    VALUES (?1, ?2, ?3)
                ON CONFLICT(mac_address) DO UPDATE SET
                    name = excluded.name,
                    model = excluded.model,
                    created_at = strftime('%s')"#,
        (
            &paired_device.name,
            SqliteMacAddr6(paired_device.mac_address),
            SqliteDeviceModel(paired_device.model),
        ),
    )?;
    Ok(())
}

pub fn delete(connection: &Connection, mac_address: MacAddr6) -> Result<(), StorageError> {
    connection.execute(
        "DELETE FROM paired_device WHERE mac_address = ?1",
        (SqliteMacAddr6(mac_address),),
    )?;
    Ok(())
}
