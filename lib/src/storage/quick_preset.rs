use rusqlite::Connection;

use crate::soundcore_device::device_model::DeviceModel;

use super::{type_conversions::SqliteDeviceModel, StorageError};

pub fn upsert(
    connection: &Connection,
    model: DeviceModel,
    name: String,
    json: String,
) -> Result<(), StorageError> {
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
        r#"DELETE FROM quick_preset WHERE model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), name),
    )?;
    Ok(())
}
