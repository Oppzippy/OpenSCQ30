use std::collections::HashMap;

use rusqlite::Connection;

use crate::{
    api::settings::{self, SettingId},
    soundcore_device::device_model::DeviceModel,
};

use super::{StorageError, type_conversions::SqliteDeviceModel};

type SettingsCollection = HashMap<SettingId<'static>, settings::Value>;

pub fn fetch(
    connection: &Connection,
    model: DeviceModel,
    name: String,
) -> Result<SettingsCollection, StorageError> {
    let json: String = connection.query_row(
        r#"SELECT settings FROM quick_preset WHERE device_model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), name),
        |row| row.get(0),
    )?;
    let settings = serde_json::from_str(&json).map_err(StorageError::JsonError)?;
    Ok(settings)
}

pub fn fetch_all(
    connection: &Connection,
    model: DeviceModel,
) -> Result<HashMap<String, SettingsCollection>, StorageError> {
    let mut query = connection.prepare_cached(
        r#"SELECT name, settings FROM quick_preset WHERE device_model = ?1 ORDER BY name"#,
    )?;
    let rows = query.query([SqliteDeviceModel(model)])?;
    rows.and_then(|row| {
        let name: String = row.get(0)?;
        let json: String = row.get(1)?;
        Ok((name, json))
    })
    .map(|result| match result {
        Ok((name, json)) => {
            let settings: SettingsCollection =
                serde_json::from_str(&json).map_err(StorageError::JsonError)?;
            Ok((name, settings))
        }
        Err(err) => Err(StorageError::Other(err)),
    })
    .collect::<Result<HashMap<String, SettingsCollection>, StorageError>>()
}

pub fn upsert(
    connection: &Connection,
    model: DeviceModel,
    name: String,
    settings: SettingsCollection,
) -> Result<(), StorageError> {
    let json = serde_json::to_string(&settings).map_err(StorageError::JsonError)?;
    connection.execute(
        r#"INSERT INTO quick_preset (device_model, name, settings)
                VALUES (?1, ?2, ?3)
            ON CONFLICT(device_model, name) DO UPDATE SET
                settings = excluded.settings"#,
        (SqliteDeviceModel(model), name, json),
    )?;
    Ok(())
}

pub fn delete(
    connection: &Connection,
    model: DeviceModel,
    name: String,
) -> Result<(), StorageError> {
    connection.execute(
        r#"DELETE FROM quick_preset WHERE device_model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), name),
    )?;
    Ok(())
}
