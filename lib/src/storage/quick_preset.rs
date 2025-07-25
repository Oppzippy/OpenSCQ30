use std::{collections::HashMap, panic::Location, str::FromStr};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::{
    api::settings::{self, SettingId},
    devices::DeviceModel,
};

use super::{Error, type_conversions::SqliteDeviceModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickPreset {
    pub name: String,
    pub fields: Vec<QuickPresetField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickPresetField {
    pub setting_id: SettingId,
    pub value: settings::Value,
    pub is_enabled: bool,
}

pub fn fetch(
    connection: &Connection,
    model: DeviceModel,
    name: String,
) -> Result<QuickPreset, Error> {
    let json: String = connection.query_row(
        r#"SELECT json(fields) FROM quick_preset WHERE device_model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), &name),
        |row| row.get(0),
    )?;
    let fields = serde_json::from_str(&json).map_err(Error::from)?;
    Ok(QuickPreset { name, fields })
}

#[tracing::instrument]
pub fn fetch_all(connection: &Connection, model: DeviceModel) -> Result<Vec<QuickPreset>, Error> {
    let mut query = connection
        .prepare_cached(r#"SELECT name, json(fields) FROM quick_preset WHERE device_model = ?1"#)?;
    let rows = query.query([SqliteDeviceModel(model)])?;
    rows.and_then(|row| {
        let name: String = row.get(0)?;
        let json: String = row.get(1)?;
        Ok((name, json))
    })
    .filter_map(
        |result: Result<(String, String), rusqlite::Error>| match result {
            Ok((name, json)) => {
                match serde_json::from_str::<Vec<QuickPresetField>>(&json).map_err(Error::from) {
                    Ok(fields) => Some(Ok(QuickPreset { name, fields })),
                    Err(err) => {
                        tracing::error!(
                            message = "failed to parse quick preset json",
                            name = name,
                            json = json,
                            error = ?err,
                        );
                        None
                    }
                }
            }
            Err(err) => Some(Err(Error::from(err))),
        },
    )
    .collect::<Result<Vec<QuickPreset>, Error>>()
}

/// If the QuickPreset does not exist yet, inserts it as is.
/// If it does exist, the fields will be replaced while retaining whether or not each field is enabled.
pub fn upsert(
    connection: &mut Connection,
    model: DeviceModel,
    mut quick_preset: QuickPreset,
) -> Result<(), Error> {
    // Ensure that the currently enabled fields can't change between when we read them and write them back
    let tx = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
    let enabled_fields = enabled_fields(&tx, model, &quick_preset.name)?;

    for field in &mut quick_preset.fields {
        if enabled_fields
            .get(&field.setting_id)
            .copied()
            .unwrap_or_default()
        {
            field.is_enabled = true;
        }
    }
    let fields_json = serde_json::to_string(&quick_preset.fields)?;

    tx.execute(
        r#"INSERT INTO quick_preset (device_model, name, fields)
                VALUES (?1, ?2, jsonb(?3))
            ON CONFLICT(device_model, name) DO UPDATE SET
                fields = excluded.fields"#,
        (SqliteDeviceModel(model), quick_preset.name, fields_json),
    )?;
    tx.commit()?;
    Ok(())
}

fn enabled_fields(
    connection: &Connection,
    model: DeviceModel,
    name: &str,
) -> Result<HashMap<SettingId, bool>, Error> {
    let mut statement = connection.prepare_cached(
        r#"SELECT
                value ->> 'settingId',
                value ->> 'isEnabled'
            FROM
                quick_preset, json_each(quick_preset.fields)
            WHERE
                device_model = ?1 AND name = ?2"#,
    )?;
    statement
        .query_map((SqliteDeviceModel(model), name), |row| {
            let setting_id_string: String = row.get(0)?;
            let is_enabled: bool = row.get(1)?;
            Ok((setting_id_string, is_enabled))
        })?
        .map(|result| match result {
            Ok((setting_id_string, is_enabled)) => {
                let setting_id: SettingId =
                    SettingId::from_str(&setting_id_string).map_err(Error::from)?;
                Ok((setting_id, is_enabled))
            }
            Err(err) => Err(Error::from(err)),
        })
        .collect::<Result<HashMap<_, _>, _>>()
}

pub fn toggle_field(
    connection: &Connection,
    model: DeviceModel,
    name: String,
    setting_id: SettingId,
    is_enabled: bool,
) -> Result<(), Error> {
    // Find the index of the field, then set isEnabled for that index
    let changed_rows = connection.execute(
        r#"
        WITH target_field_index AS (
            SELECT
                key
            FROM
                quick_preset, json_each(quick_preset.fields)
            WHERE
                device_model = ?1 AND name = ?2 AND
                value ->> 'settingId' = ?3
        )
        UPDATE quick_preset
        SET
            fields = jsonb_replace(fields, '$[' || (SELECT key FROM target_field_index) || '].isEnabled', json(?4))
        WHERE
            device_model = ?1 AND name = ?2"#,
        (
            SqliteDeviceModel(model),
            name,
            setting_id.to_string(),
            if is_enabled { "true" } else { "false" },
        ),
    )?;

    // In case something went wrong with the where clause, causing 0 rows to be found
    if changed_rows == 1 {
        Ok(())
    } else {
        Err(Error::NotFound {
            location: Location::caller(),
        })
    }
}

pub fn delete(connection: &Connection, model: DeviceModel, name: String) -> Result<(), Error> {
    connection.execute(
        r#"DELETE FROM quick_preset WHERE device_model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), name),
    )?;
    Ok(())
}
