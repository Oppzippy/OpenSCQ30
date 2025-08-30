use std::panic::Location;

use rusqlite::Connection;

use crate::devices::DeviceModel;

use super::{Error, type_conversions::SqliteDeviceModel};

pub fn fetch(connection: &Connection, model: DeviceModel, name: String) -> Result<Vec<i16>, Error> {
    let mut query = connection.prepare_cached(
        r#"SELECT volume_adjustments FROM equalizer_profile WHERE device_model = ?1 AND name = ?2"#,
    )?;
    let mut rows = query.query_and_then(
        (SqliteDeviceModel(model), name),
        |row| -> Result<_, Error> {
            let json = row.get_ref(0)?.as_str()?;
            let volume_adjustments: Vec<i16> = serde_json::from_str(json)?;
            Ok(volume_adjustments)
        },
    )?;
    rows.next()
        .ok_or(Error::NotFound {
            location: Location::caller(),
        })
        .flatten()
}

pub fn fetch_all(
    connection: &Connection,
    model: DeviceModel,
) -> Result<Vec<(String, Vec<i16>)>, Error> {
    let mut query = connection.prepare_cached(
        r#"SELECT name, volume_adjustments FROM equalizer_profile WHERE device_model = ?1 ORDER BY name"#,
    )?;
    let rows = query.query([SqliteDeviceModel(model)])?;
    rows.and_then(|row| -> Result<_, Error> {
        let name = row.get(0)?;
        let volume_adjustments_json = row.get_ref(1)?.as_str()?;
        let volume_adjustments: Vec<i16> = serde_json::from_str(volume_adjustments_json)?;
        Ok((name, volume_adjustments))
    })
    .collect::<Result<Vec<_>, _>>()
}

pub fn upsert(
    connection: &Connection,
    model: DeviceModel,
    name: String,
    volume_adjustments: Vec<i16>,
) -> Result<(), Error> {
    let json = serde_json::to_string(&volume_adjustments).map_err(Error::from)?;
    // by calling sqlite's json(...) function, we ensure it is minified so that formatting is standardized, making it okay to
    // perform equality comparisons. this is necessary for the unique index on volume_adjustments.
    connection.execute(
        r#"INSERT INTO equalizer_profile (device_model, name, volume_adjustments)
                VALUES (?1, ?2, json(?3))
            ON CONFLICT(device_model, name) DO UPDATE SET
                volume_adjustments = excluded.volume_adjustments
            ON CONFLICT(device_model, volume_adjustments) DO UPDATE SET
                name = excluded.name
                "#,
        (SqliteDeviceModel(model), name, json),
    )?;
    Ok(())
}

pub fn bulk_upsert(
    connection: &mut Connection,
    model: DeviceModel,
    profiles: Vec<(String, Vec<i16>)>,
) -> Result<(), Error> {
    let tx = connection.transaction()?;
    for (name, volume_adjustments) in profiles {
        upsert(&tx, model, name, volume_adjustments)?;
    }
    tx.commit()?;
    Ok(())
}

pub fn delete(connection: &Connection, model: DeviceModel, name: String) -> Result<(), Error> {
    connection.execute(
        r#"DELETE FROM equalizer_profile WHERE device_model = ?1 AND name = ?2"#,
        (SqliteDeviceModel(model), name),
    )?;
    Ok(())
}
