use rusqlite::{Connection, OptionalExtension};

use crate::{api::settings::SettingId, devices::DeviceModel};

use super::{Error, type_conversions::SqliteDeviceModel};

pub fn fetch(
    connection: &Connection,
    model: DeviceModel,
    setting_id: SettingId,
) -> Result<Option<serde_json::Value>, Error> {
    let setting_id: &'static str = setting_id.into();
    let json: Option<String> = connection
        .prepare_cached(
            r#"SELECT value FROM device_model_setting WHERE device_model = ?1 AND setting_id = ?2"#,
        )?
        .query_row((SqliteDeviceModel(model), setting_id), |row| row.get(0))
        .optional()?;
    json.map(|json| serde_json::from_str(&json).map_err(Error::from))
        .transpose()
}

pub fn upsert(
    connection: &Connection,
    model: DeviceModel,
    setting_id: SettingId,
    value: serde_json::Value,
) -> Result<(), Error> {
    let setting_id: &'static str = setting_id.into();
    let json = serde_json::to_string(&value).map_err(Error::from)?;
    connection.execute(
        r#"INSERT INTO device_model_setting (device_model, setting_id, value)
                VALUES (?1, ?2, json(?3))
            ON CONFLICT(device_model, setting_id) DO UPDATE SET
                value = excluded.value
        "#,
        (SqliteDeviceModel(model), setting_id, json),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{api::settings::SettingId, devices::DeviceModel, storage::OpenSCQ30Database};

    #[tokio::test]
    async fn missing_setting_is_none() {
        let database = OpenSCQ30Database::new_in_memory().await.unwrap();
        let value = database
            .fetch_device_model_setting(DeviceModel::SoundcoreD1301, SettingId::DualConnections)
            .await
            .unwrap();
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn upsert_overwrites_and_is_per_model() {
        let database = OpenSCQ30Database::new_in_memory().await.unwrap();
        for value in [true, false] {
            database
                .upsert_device_model_setting(
                    DeviceModel::SoundcoreD1301,
                    SettingId::DualConnections,
                    value.into(),
                )
                .await
                .unwrap();
        }
        assert_eq!(
            database
                .fetch_device_model_setting(DeviceModel::SoundcoreD1301, SettingId::DualConnections)
                .await
                .unwrap(),
            Some(false.into()),
        );
        assert_eq!(
            database
                .fetch_device_model_setting(DeviceModel::SoundcoreA3040, SettingId::DualConnections)
                .await
                .unwrap(),
            None,
        );
    }
}
