use std::collections::HashMap;

use rusqlite::Connection;

use crate::{
    api::settings::{self, SettingId},
    devices::DeviceModel,
};

use super::{Error, type_conversions::SqliteDeviceModel};

type SettingsCollection = HashMap<SettingId, settings::Value>;

pub fn fetch(
    connection: &Connection,
    model: DeviceModel,
    name: String,
) -> Result<SettingsCollection, Error> {
    let json: String = connection.query_row(
        r#"SELECT settings FROM quick_preset WHERE device_model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), name),
        |row| row.get(0),
    )?;
    let settings = serde_json::from_str(&json).map_err(Error::from)?;
    Ok(settings)
}

pub fn fetch_all(
    connection: &Connection,
    model: DeviceModel,
) -> Result<HashMap<String, SettingsCollection>, Error> {
    let mut query = connection
        .prepare_cached(r#"SELECT name, settings FROM quick_preset WHERE device_model = ?1"#)?;
    let rows = query.query([SqliteDeviceModel(model)])?;
    rows.and_then(|row| {
        let name: String = row.get(0)?;
        let json: String = row.get(1)?;
        Ok((name, json))
    })
    .map(
        |result: Result<(String, String), rusqlite::Error>| match result {
            Ok((name, json)) => {
                let settings: SettingsCollection =
                    serde_json::from_str(&json).map_err(Error::from)?;
                Ok((name, settings))
            }
            Err(err) => Err(Error::from(err)),
        },
    )
    .collect::<Result<HashMap<String, SettingsCollection>, Error>>()
}

pub fn upsert(
    connection: &Connection,
    model: DeviceModel,
    name: String,
    settings: SettingsCollection,
) -> Result<(), Error> {
    let json = serde_json::to_string(&settings).map_err(Error::from)?;
    connection.execute(
        r#"INSERT INTO quick_preset (device_model, name, settings)
                VALUES (?1, ?2, ?3)
            ON CONFLICT(device_model, name) DO UPDATE SET
                settings = excluded.settings"#,
        (SqliteDeviceModel(model), name, json),
    )?;
    Ok(())
}

pub fn delete(connection: &Connection, model: DeviceModel, name: String) -> Result<(), Error> {
    connection.execute(
        r#"DELETE FROM quick_preset WHERE device_model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), name),
    )?;
    Ok(())
}
