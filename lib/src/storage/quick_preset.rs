use std::{collections::HashMap, mem, panic::Location, str::FromStr};

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
    let mut query = connection.prepare_cached(
        r#"SELECT json(value) FROM quick_preset, json_each(fields) WHERE device_model = ?1 AND name = ?2"#,
    )?;
    let fields = query
        .query_map((SqliteDeviceModel(model), &name), |row| {
            let json = row.get_ref(0)?.as_str()?;
            match serde_json::from_str::<QuickPresetField>(json).map_err(Error::from) {
                Ok(field) => Ok(Some(field)),
                Err(err) => {
                    // Log and ignore invalid fields. This is done so that things don't break when, for example, an
                    // option is removed from a Setting::Select. We don't remove the invalid field from the database so
                    // that no data is lost if the field becomes valid again in the future.
                    tracing::warn!(
                        message = "failed to parse quick preset field, skipping",
                        quick_preset_name = name,
                        field_json = json,
                        error = ?err,
                    );
                    Ok(None)
                }
            }
        })?
        .filter_map(|result| result.transpose())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(QuickPreset { name, fields })
}

#[tracing::instrument]
pub fn fetch_all(connection: &Connection, model: DeviceModel) -> Result<Vec<QuickPreset>, Error> {
    let mut query = connection.prepare_cached(
        r#"SELECT name, json(value) as fields FROM quick_preset, json_each(fields) WHERE device_model = ?1 ORDER BY name"#,
    )?;
    let mut rows = query.query([SqliteDeviceModel(model)])?;

    let mut quick_presets = Vec::new();
    let mut preset_name: Option<String> = None;
    let mut preset_fields = Vec::new();
    while let Some(row) = rows.next().transpose() {
        let row = row?;
        let current_name = row.get_ref(0)?.as_str()?;
        let current_json = row.get_ref(1)?.as_str()?;
        if preset_name.is_none() {
            preset_name = Some(current_name.to_owned());
        } else if preset_name.as_ref().map(|s| s.as_str()) != Some(current_name) {
            let group_name = preset_name
                .replace(current_name.to_owned())
                .expect("the previous if statement covers the none case");
            quick_presets.push(QuickPreset {
                name: group_name.to_owned(),
                fields: mem::take(&mut preset_fields),
            });
        }

        match serde_json::from_str::<QuickPresetField>(current_json).map_err(Error::from) {
            Ok(field) => preset_fields.push(field),
            Err(err) => {
                // Log and ignore invalid fields. This is done so that things don't break when, for example, an
                // option is removed from a Setting::Select. We don't remove the invalid field from the database so
                // that no data is lost if the field becomes valid again in the future.
                tracing::warn!(
                    message = "failed to parse quick preset field, skipping",
                    quick_preset_name = current_name,
                    field_json = current_json,
                    error = ?err,
                );
            }
        }
    }
    if !preset_fields.is_empty() {
        quick_presets.push(QuickPreset {
            name: preset_name
                .take()
                .expect("if there is a field, there must also be a name"),
            fields: preset_fields,
        });
    }
    Ok(quick_presets)
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
