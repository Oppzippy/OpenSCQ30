mod migration;
mod type_conversions;

use std::rc::Rc;

use macaddr::MacAddr6;
use openscq30_lib::soundcore_device::device_model::DeviceModel;
use rusqlite::{ffi::SQLITE_CONSTRAINT_UNIQUE, Connection};
use thiserror::Error;
use tracing::{instrument, warn};
use type_conversions::{SqliteDeviceModel, SqliteMacAddr6};

#[derive(Clone)]
pub struct OpenSCQ30Database {
    connection: Rc<Connection>,
}

#[derive(Clone, Debug)]
pub struct PairedDevice {
    pub name: String,
    pub mac_address: MacAddr6,
    pub model: DeviceModel,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("sql error: {0:?}")]
    AlreadyExists(rusqlite::Error),
    #[error("sql error: {0:?}")]
    Other(rusqlite::Error),
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        if let Some(sqlite_err) = err.sqlite_error() {
            #[allow(non_snake_case)]
            match sqlite_err.extended_code {
                SQLITE_CONSTRAINT_UNIQUE => return Error::AlreadyExists(err),
                _ => (),
            }
        }
        Error::Other(err)
    }
}

impl OpenSCQ30Database {
    pub fn new() -> Result<Self, Error> {
        let mut connection = Connection::open_in_memory()?;
        migration::migrate(&mut connection, migration::MIGRATIONS)?;
        Ok(Self {
            connection: Rc::new(connection),
        })
    }

    #[instrument(skip(self))]
    pub fn fetch_paired_devices(&self) -> Result<Vec<PairedDevice>, Error> {
        let mut statement = self.connection.prepare_cached(
            "SELECT name, mac_address, model FROM paired_device ORDER BY name ASC",
        )?;
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

    pub fn insert_paired_device(&self, paired_device: &PairedDevice) -> Result<(), Error> {
        self.connection.execute(
            "INSERT INTO paired_device (name, mac_address, model) VALUES (?1, ?2, ?3)",
            (
                &paired_device.name,
                SqliteMacAddr6(paired_device.mac_address),
                SqliteDeviceModel(paired_device.model),
            ),
        )?;
        Ok(())
    }

    pub fn upsert_paired_device(&self, paired_device: &PairedDevice) -> Result<(), Error> {
        self.connection.execute(
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

    pub fn delete_paired_device(&self, mac_address: MacAddr6) -> Result<(), Error> {
        self.connection.execute(
            "DELETE FROM paired_device WHERE mac_address = ?1",
            (SqliteMacAddr6(mac_address),),
        )?;
        Ok(())
    }

    pub fn upsert_quick_preset(
        &self,
        model: DeviceModel,
        name: &str,
        json: &str,
    ) -> Result<(), Error> {
        self.connection.execute(
            r#"INSERT INTO quick_preset (device_model, name, settings)
                VALUES (?1, ?2, ?3)
            ON CONFLICT(device_model, name) DO UPDATE SET
                settings = excluded.settings"#,
            (SqliteDeviceModel(model), name, json),
        )?;
        Ok(())
    }

    pub fn delete_quick_preset(&self, model: DeviceModel, name: &str) -> Result<(), Error> {
        self.connection.execute(
            r#"DELETE FROM quick_preset WHERE model = ?1 AND name = ?2"#,
            (SqliteDeviceModel(model), name),
        )?;
        Ok(())
    }
}
