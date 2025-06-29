use std::sync::Arc;

use macaddr::MacAddr6;
use openscq30_i18n_macros::Translate;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr, VariantArray};

use crate::{
    api::device::{self, OpenSCQ30DeviceRegistry},
    connection_backend::ConnectionBackends,
    devices::soundcore,
    storage::OpenSCQ30Database,
};

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    VariantArray,
    AsRefStr,
    Display,
    EnumIter,
    IntoStaticStr,
    EnumString,
    Serialize,
    Deserialize,
    Translate,
)]
pub enum DeviceModel {
    SoundcoreA3004,
    SoundcoreA3027,
    SoundcoreA3028,
    SoundcoreA3029,
    SoundcoreA3030,
    SoundcoreA3031,
    SoundcoreA3033,
    SoundcoreA3926,
    SoundcoreA3930,
    SoundcoreA3931,
    SoundcoreA3933,
    SoundcoreA3936,
    SoundcoreA3945,
    SoundcoreA3951,
    SoundcoreA3939,
    SoundcoreA3935,
    SoundcoreA3959,
    SoundcoreDevelopment,
}

impl DeviceModel {
    pub async fn device_registry<B: ConnectionBackends + 'static>(
        &self,
        backends: &B,
        database: Arc<OpenSCQ30Database>,
    ) -> device::Result<Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>> {
        macro_rules! new_soundcore_device {
            ($($module:tt)*) => {
                Ok(Arc::new($($module)*::device_registry::<B::Rfcomm>(
                    backends.rfcomm().await?,
                    database,
                    *self,
                )))
            };
        }
        match self {
            DeviceModel::SoundcoreA3004 => new_soundcore_device!(soundcore::a3004),
            DeviceModel::SoundcoreA3027 | DeviceModel::SoundcoreA3030 => {
                new_soundcore_device!(soundcore::a3027)
            }
            DeviceModel::SoundcoreA3028 | DeviceModel::SoundcoreA3029 => {
                new_soundcore_device!(soundcore::a3028)
            }
            DeviceModel::SoundcoreA3031 => new_soundcore_device!(soundcore::a3031),
            DeviceModel::SoundcoreA3033 => new_soundcore_device!(soundcore::a3033),
            DeviceModel::SoundcoreA3926 => new_soundcore_device!(soundcore::a3926),
            DeviceModel::SoundcoreA3930 => new_soundcore_device!(soundcore::a3930),
            DeviceModel::SoundcoreA3931 | DeviceModel::SoundcoreA3935 => {
                new_soundcore_device!(soundcore::a3931)
            }
            DeviceModel::SoundcoreA3933 | DeviceModel::SoundcoreA3939 => {
                new_soundcore_device!(soundcore::a3933)
            }
            DeviceModel::SoundcoreA3936 => new_soundcore_device!(soundcore::a3936),
            DeviceModel::SoundcoreA3945 => new_soundcore_device!(soundcore::a3945),
            DeviceModel::SoundcoreA3951 => new_soundcore_device!(soundcore::a3951),
            DeviceModel::SoundcoreA3959 => new_soundcore_device!(soundcore::a3959),
            DeviceModel::SoundcoreDevelopment => new_soundcore_device!(soundcore::development),
        }
    }

    pub async fn demo_device_registry(
        &self,
        database: Arc<OpenSCQ30Database>,
    ) -> device::Result<Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>> {
        macro_rules! new_soundcore_device {
            ($($module:tt)*) => {
                Ok(Arc::new($($module)*::demo_device_registry(database, *self)))
            };
        }
        match self {
            DeviceModel::SoundcoreA3004 => new_soundcore_device!(soundcore::a3004),

            DeviceModel::SoundcoreA3027 | DeviceModel::SoundcoreA3030 => {
                new_soundcore_device!(soundcore::a3027)
            }
            DeviceModel::SoundcoreA3028 | DeviceModel::SoundcoreA3029 => {
                new_soundcore_device!(soundcore::a3028)
            }
            DeviceModel::SoundcoreA3031 => new_soundcore_device!(soundcore::a3031),
            DeviceModel::SoundcoreA3033 => new_soundcore_device!(soundcore::a3033),
            DeviceModel::SoundcoreA3926 => new_soundcore_device!(soundcore::a3926),
            DeviceModel::SoundcoreA3930 => new_soundcore_device!(soundcore::a3930),
            DeviceModel::SoundcoreA3931 | DeviceModel::SoundcoreA3935 => {
                new_soundcore_device!(soundcore::a3931)
            }
            DeviceModel::SoundcoreA3933 | DeviceModel::SoundcoreA3939 => {
                new_soundcore_device!(soundcore::a3933)
            }
            DeviceModel::SoundcoreA3936 => new_soundcore_device!(soundcore::a3936),
            DeviceModel::SoundcoreA3945 => new_soundcore_device!(soundcore::a3945),
            DeviceModel::SoundcoreA3951 => new_soundcore_device!(soundcore::a3951),
            DeviceModel::SoundcoreA3959 => new_soundcore_device!(soundcore::a3959),
            DeviceModel::SoundcoreDevelopment => new_soundcore_device!(soundcore::development),
        }
    }

    pub fn demo_mac_address(&self) -> MacAddr6 {
        let index = Self::VARIANTS
            .iter()
            .position(|variant| variant == self)
            .unwrap_or_default();
        let mac_address_bytes: [u8; 6] = (index as u64).to_be_bytes()[2..].try_into().unwrap();
        MacAddr6::from(mac_address_bytes)
    }
}
