use std::{collections::HashMap, mem, panic::Location, str::FromStr};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::{
    api::settings::{self, SettingId},
    devices::DeviceModel,
};

use super::{Error, type_conversions::SqliteDeviceModel};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickPreset {
    pub name: String,
    pub fields: Vec<QuickPresetField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    let mut query = connection
        .prepare_cached(r#"SELECT 1 FROM quick_preset WHERE device_model = ?1 AND name = ?2"#)?;
    if query
        .query((SqliteDeviceModel(model), &name))?
        .next()?
        .is_none()
    {
        return Err(Error::NotFound {
            location: Location::caller(),
        });
    }

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
    // The LEFT JOIN ensures that we still get the names of quick presets with no fields
    let mut query = connection.prepare_cached(
        r#"SELECT name, json(value) as fields FROM quick_preset LEFT JOIN json_each(fields) WHERE device_model = ?1 ORDER BY name"#,
    )?;
    let mut rows = query.query([SqliteDeviceModel(model)])?;

    let mut quick_presets = Vec::new();
    let mut preset_name: Option<String> = None;
    let mut preset_fields = Vec::new();
    while let Some(row) = rows.next().transpose() {
        let row = row?;
        let current_name = row.get_ref(0)?.as_str()?;
        let Some(current_json) = row.get_ref(1)?.as_str_or_null()? else {
            // This will only be None for quick presets that have 0 fields, thanks to the LEFT JOIN
            quick_presets.push(QuickPreset {
                name: current_name.to_owned(),
                fields: Vec::new(),
            });
            continue;
        };
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
        .query_and_then(
            (SqliteDeviceModel(model), name),
            |row: &rusqlite::Row| -> Result<_, Error> {
                let setting_id = SettingId::from_str(row.get_ref(0)?.as_str()?)?;
                let is_enabled: bool = row.get(1)?;
                Ok((setting_id, is_enabled))
            },
        )?
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
            device_model = ?1 AND name = ?2 AND (SELECT COUNT(*) FROM target_field_index) = 1"#,
        (
            SqliteDeviceModel(model),
            name,
            <SettingId as Into<&'static str>>::into(setting_id),
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

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::HashSet, hash::RandomState};

    use crate::{api::settings::Value, storage::OpenSCQ30Database};

    use super::*;

    fn test_data() -> Vec<QuickPreset> {
        vec![
            QuickPreset {
                name: "Preset 1".into(),
                fields: vec![
                    QuickPresetField {
                        setting_id: SettingId::AmbientSoundMode,
                        value: Cow::from("normal").into(),
                        is_enabled: true,
                    },
                    QuickPresetField {
                        setting_id: SettingId::NoiseCancelingMode,
                        value: Cow::from("indoor").into(),
                        is_enabled: false,
                    },
                ],
            },
            QuickPreset {
                name: "Preset 2".into(),
                fields: vec![
                    QuickPresetField {
                        setting_id: SettingId::AmbientSoundMode,
                        value: Value::I32(5),
                        is_enabled: true,
                    },
                    QuickPresetField {
                        setting_id: SettingId::NoiseCancelingMode,
                        value: Value::OptionalString(Some("Asdf".into())),
                        is_enabled: false,
                    },
                ],
            },
            QuickPreset {
                name: "Preset 3".into(),
                fields: vec![],
            },
            QuickPreset {
                name: "Preset 4".into(),
                fields: vec![
                    QuickPresetField {
                        setting_id: SettingId::ExportCustomEqualizerProfilesOutput,
                        value: Value::StringVec(vec!["1".into(), "2".into()]),
                        is_enabled: true,
                    },
                    QuickPresetField {
                        setting_id: SettingId::NoiseCancelingMode,
                        value: Value::I32(2),
                        is_enabled: false,
                    },
                ],
            },
            QuickPreset {
                name: "Preset 5".into(),
                fields: vec![],
            },
        ]
    }

    #[tokio::test]
    async fn test_fetch_all() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let test_data = test_data();
        for preset in &test_data {
            db.upsert_quick_preset(DeviceModel::SoundcoreA3004, preset.clone())
                .await
                .unwrap();
        }
        // insert one for another device to ensure it is excluded from the results
        db.upsert_quick_preset(
            DeviceModel::SoundcoreA3028,
            QuickPreset {
                name: "Preset 1".into(),
                fields: vec![QuickPresetField {
                    setting_id: SettingId::LeftDoublePress,
                    value: 100.into(),
                    is_enabled: true,
                }],
            },
        )
        .await
        .unwrap();

        let fetched_presets = db
            .fetch_all_quick_presets(DeviceModel::SoundcoreA3004)
            .await
            .unwrap();
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(test_data),
            HashSet::from_iter(fetched_presets)
        );
    }

    #[tokio::test]
    async fn test_fetch() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let test_data = test_data();
        for preset in &test_data {
            db.upsert_quick_preset(DeviceModel::SoundcoreA3004, preset.clone())
                .await
                .unwrap();
        }
        // insert one for another device to ensure it is excluded from the results
        db.upsert_quick_preset(
            DeviceModel::SoundcoreA3028,
            QuickPreset {
                name: "Preset 1".into(),
                fields: vec![QuickPresetField {
                    setting_id: SettingId::LeftDoublePress,
                    value: 100.into(),
                    is_enabled: true,
                }],
            },
        )
        .await
        .unwrap();

        let fetched_preset = db
            .fetch_quick_preset(DeviceModel::SoundcoreA3004, "Preset 1".into())
            .await
            .unwrap();
        assert_eq!(test_data[0], fetched_preset);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let test_data = test_data();
        for preset in &test_data {
            db.upsert_quick_preset(DeviceModel::SoundcoreA3004, preset.clone())
                .await
                .unwrap();
        }
        // insert one for another device to ensure it is excluded from the results
        db.upsert_quick_preset(
            DeviceModel::SoundcoreA3028,
            QuickPreset {
                name: "Preset 1".into(),
                fields: vec![QuickPresetField {
                    setting_id: SettingId::LeftDoublePress,
                    value: 100.into(),
                    is_enabled: true,
                }],
            },
        )
        .await
        .unwrap();

        db.delete_quick_preset(DeviceModel::SoundcoreA3004, "Preset 1".into())
            .await
            .unwrap();
        let fetch_one_err = db
            .fetch_quick_preset(DeviceModel::SoundcoreA3004, "Preset 1".into())
            .await
            .unwrap_err();
        assert!(
            matches!(fetch_one_err, Error::NotFound { .. }),
            "wanted not found, got {fetch_one_err:?}",
        );

        let fetched_presets = db
            .fetch_all_quick_presets(DeviceModel::SoundcoreA3004)
            .await
            .unwrap();
        assert!(
            fetched_presets.len() > 0,
            "the other presets for the same device should not have been deleted",
        );
        assert!(
            fetched_presets
                .iter()
                .all(|preset| preset.name != "Preset 1"),
            "the preset should not show up when fetching all presets",
        );

        db.fetch_quick_preset(DeviceModel::SoundcoreA3028, "Preset 1".into())
            .await
            .expect("the other device's preset with the same name should not have been deleted");
    }

    #[tokio::test]
    async fn test_upsert() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        db.upsert_quick_preset(
            DeviceModel::SoundcoreA3004,
            QuickPreset {
                name: "Preset 1".into(),
                fields: vec![
                    QuickPresetField {
                        setting_id: SettingId::AmbientSoundMode,
                        value: Cow::from("normal").into(),
                        is_enabled: true,
                    },
                    QuickPresetField {
                        setting_id: SettingId::NoiseCancelingMode,
                        value: Cow::from("indoor").into(),
                        is_enabled: false,
                    },
                    QuickPresetField {
                        setting_id: SettingId::TransparencyMode,
                        value: Cow::from("fullyTransparent").into(),
                        is_enabled: false,
                    },
                ],
            },
        )
        .await
        .unwrap();

        db.upsert_quick_preset(
            DeviceModel::SoundcoreA3004,
            QuickPreset {
                name: "Preset 1".into(),
                fields: vec![
                    QuickPresetField {
                        setting_id: SettingId::AmbientSoundMode,
                        // change to transparency
                        value: Cow::from("transparency").into(),
                        // change to disabled, but this will be ignored
                        is_enabled: false,
                    },
                    QuickPresetField {
                        setting_id: SettingId::NoiseCancelingMode,
                        value: Cow::from("indoor").into(),
                        // change to enabled
                        is_enabled: true,
                    },
                    // remove transparency mode field
                    // add a new field
                    QuickPresetField {
                        setting_id: SettingId::BatteryLevel,
                        value: 5.into(),
                        is_enabled: false,
                    },
                ],
            },
        )
        .await
        .unwrap();

        let preset = db
            .fetch_quick_preset(DeviceModel::SoundcoreA3004, "Preset 1".into())
            .await
            .expect("the other device's preset with the same name should not have been deleted");
        assert_eq!(
            preset,
            QuickPreset {
                name: "Preset 1".into(),
                fields: vec![
                    QuickPresetField {
                        setting_id: SettingId::AmbientSoundMode,
                        value: Cow::from("transparency").into(),
                        is_enabled: true,
                    },
                    QuickPresetField {
                        setting_id: SettingId::NoiseCancelingMode,
                        value: Cow::from("indoor").into(),
                        is_enabled: true,
                    },
                    QuickPresetField {
                        setting_id: SettingId::BatteryLevel,
                        value: 5.into(),
                        is_enabled: false,
                    },
                ],
            },
        );
    }

    #[tokio::test]
    async fn test_toggle_field() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let test_data = test_data();
        for preset in &test_data {
            db.upsert_quick_preset(DeviceModel::SoundcoreA3004, preset.clone())
                .await
                .unwrap();
        }
        // insert one for another device to ensure it is not changed
        db.upsert_quick_preset(DeviceModel::SoundcoreA3028, test_data[0].clone())
            .await
            .unwrap();

        db.toggle_quick_preset_field(
            DeviceModel::SoundcoreA3004,
            "Preset 1".into(),
            SettingId::AmbientSoundMode,
            false,
        )
        .await
        .unwrap();

        let fetched_preset = db
            .fetch_quick_preset(DeviceModel::SoundcoreA3004, "Preset 1".into())
            .await
            .unwrap();
        assert_eq!(
            test_data[0]
                .fields
                .iter()
                .find(|entry| entry.setting_id == SettingId::AmbientSoundMode)
                .unwrap()
                .is_enabled,
            true,
            "Test data is not as expected",
        );
        assert_eq!(
            fetched_preset
                .fields
                .iter()
                .find(|entry| entry.setting_id == SettingId::AmbientSoundMode)
                .unwrap()
                .is_enabled,
            false,
        );

        let other_device_fetched_preset = db
            .fetch_quick_preset(DeviceModel::SoundcoreA3028, "Preset 1".into())
            .await
            .unwrap();
        assert_eq!(
            test_data[0], other_device_fetched_preset,
            "the other device's quick preset should not be modified",
        );
    }

    #[tokio::test]
    async fn test_toggle_field_on_nonexistant_preset() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let test_data = test_data();
        for preset in &test_data {
            db.upsert_quick_preset(DeviceModel::SoundcoreA3004, preset.clone())
                .await
                .unwrap();
        }

        let err = db
            .toggle_quick_preset_field(
                DeviceModel::SoundcoreA3004,
                "Preset Does Not Exist".into(),
                SettingId::AmbientSoundMode,
                false,
            )
            .await
            .unwrap_err();

        assert!(
            matches!(err, Error::NotFound { .. }),
            "should be not found: {err:?}",
        );
    }

    #[tokio::test]
    async fn test_toggle_field_on_nonexistant_field() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let test_data = test_data();
        for preset in &test_data {
            db.upsert_quick_preset(DeviceModel::SoundcoreA3004, preset.clone())
                .await
                .unwrap();
        }

        let err = db
            .toggle_quick_preset_field(
                DeviceModel::SoundcoreA3004,
                "Preset 1".into(),
                SettingId::FirmwareVersionLeft,
                false,
            )
            .await
            .unwrap_err();

        assert!(
            matches!(err, Error::NotFound { .. }),
            "should be not found: {err:?}",
        );
    }
}
