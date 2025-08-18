use macaddr::MacAddr6;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};

use crate::devices::DeviceModel;

use super::{
    Error,
    type_conversions::{SqliteDeviceModel, SqliteMacAddr6},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairedDevice {
    #[serde(with = "crate::serialization::mac_addr")]
    pub mac_address: MacAddr6,
    pub model: DeviceModel,
    pub is_demo: bool,
}

#[instrument(skip(connection))]
pub fn fetch_all(connection: &Connection) -> Result<Vec<PairedDevice>, Error> {
    let mut statement = connection.prepare_cached(
        "SELECT mac_address, model, is_demo FROM paired_device ORDER BY model ASC",
    )?;
    let devices = statement
        .query(())?
        .mapped(|row| {
            let mac_address: SqliteMacAddr6 = row.get("mac_address")?;
            let model: SqliteDeviceModel = row.get("model")?;
            let is_demo: bool = row.get("is_demo")?;
            Ok(PairedDevice {
                mac_address: mac_address.0,
                model: model.0,
                is_demo,
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
) -> Result<Option<PairedDevice>, Error> {
    let mut statement = connection.prepare_cached(
        "SELECT mac_address, model, is_demo FROM paired_device WHERE mac_address = ?1",
    )?;
    let devices = statement
        .query([SqliteMacAddr6(mac_address)])?
        .mapped(|row| {
            let mac_address: SqliteMacAddr6 = row.get("mac_address")?;
            let model: SqliteDeviceModel = row.get("model")?;
            let is_demo: bool = row.get("is_demo")?;
            Ok(PairedDevice {
                mac_address: mac_address.0,
                model: model.0,
                is_demo,
            })
        })
        .find_map(|result| match result {
            Ok(device) => Some(device),
            Err(err) => {
                warn!("error parsing row: {err:?}");
                None
            }
        });
    Ok(devices)
}

pub fn insert(connection: &Connection, paired_device: PairedDevice) -> Result<(), Error> {
    connection.execute(
        "INSERT INTO paired_device (mac_address, model, is_demo) VALUES (?1, ?2, ?3)",
        (
            SqliteMacAddr6(paired_device.mac_address),
            SqliteDeviceModel(paired_device.model),
            paired_device.is_demo,
        ),
    )?;
    Ok(())
}

pub fn upsert(connection: &Connection, paired_device: PairedDevice) -> Result<(), Error> {
    connection.execute(
        r#"INSERT INTO paired_device (mac_address, model, is_demo)
                    VALUES (?1, ?2, ?3)
                ON CONFLICT(mac_address) DO UPDATE SET
                    model = excluded.model,
                    is_demo = excluded.is_demo,
                    created_at = strftime('%s')"#,
        (
            SqliteMacAddr6(paired_device.mac_address),
            SqliteDeviceModel(paired_device.model),
            paired_device.is_demo,
        ),
    )?;
    Ok(())
}

pub fn delete(connection: &Connection, mac_address: MacAddr6) -> Result<(), Error> {
    connection.execute(
        "DELETE FROM paired_device WHERE mac_address = ?1",
        (SqliteMacAddr6(mac_address),),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use macaddr::MacAddr6;

    use crate::storage::{OpenSCQ30Database, PairedDevice};

    use super::*;

    #[tokio::test]
    async fn test_fetch_all() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let paired_devices = [
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
                model: DeviceModel::SoundcoreA3028,
                is_demo: false,
            },
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                model: DeviceModel::SoundcoreA3033,
                is_demo: false,
            },
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 3),
                model: DeviceModel::SoundcoreA3029,
                is_demo: false,
            },
        ];
        for device in &paired_devices {
            db.insert_paired_device(*device).await.unwrap();
        }
        let fetched_devices = db.fetch_all_paired_devices().await.unwrap();

        assert_eq!(
            HashSet::from(paired_devices),
            HashSet::from_iter(fetched_devices)
        );
    }

    #[tokio::test]
    async fn test_fetch() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let paired_devices = [
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
                model: DeviceModel::SoundcoreA3028,
                is_demo: false,
            },
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                model: DeviceModel::SoundcoreA3033,
                is_demo: false,
            },
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 3),
                model: DeviceModel::SoundcoreA3029,
                is_demo: false,
            },
        ];
        for device in paired_devices {
            db.insert_paired_device(device).await.unwrap();
        }

        for device in paired_devices {
            let fetched_device = db.fetch_paired_device(device.mac_address).await.unwrap();
            assert_eq!(Some(device), fetched_device);
        }
    }

    #[tokio::test]
    async fn test_upsert() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let paired_devices = [
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
                model: DeviceModel::SoundcoreA3028,
                is_demo: false,
            },
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                model: DeviceModel::SoundcoreA3033,
                is_demo: false,
            },
        ];

        // Insert one to be overwritten
        db.insert_paired_device(PairedDevice {
            mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
            model: DeviceModel::SoundcoreA3004,
            is_demo: true,
        })
        .await
        .unwrap();
        for device in &paired_devices {
            db.upsert_paired_device(*device).await.unwrap();
        }
        let fetched_devices = db.fetch_all_paired_devices().await.unwrap();

        assert_eq!(
            HashSet::from(paired_devices),
            HashSet::from_iter(fetched_devices)
        );
    }

    #[tokio::test]
    async fn test_delete() {
        let db = OpenSCQ30Database::new_in_memory().await.unwrap();
        let paired_devices = [
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
                model: DeviceModel::SoundcoreA3028,
                is_demo: false,
            },
            PairedDevice {
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                model: DeviceModel::SoundcoreA3033,
                is_demo: false,
            },
        ];
        for device in &paired_devices {
            db.upsert_paired_device(*device).await.unwrap();
        }

        // Insert one to be overwritten
        db.delete_paired_device(paired_devices[0].mac_address)
            .await
            .unwrap();
        let fetched_devices = db.fetch_all_paired_devices().await.unwrap();

        assert_eq!(
            HashSet::from([paired_devices[1]]),
            HashSet::from_iter(fetched_devices)
        );
    }
}
