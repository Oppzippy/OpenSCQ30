use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr, VariantArray};

use crate::{
    api::device::OpenSCQ30DeviceRegistry,
    connection_backend::ConnectionBackends,
    devices::{
        a3027, a3028, a3031, a3033, a3926, a3930, a3931, a3933, a3936, a3945, a3951,
        standard::structures::SerialNumber,
    },
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
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum DeviceModel {
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
}

impl DeviceModel {
    pub fn from_serial_number(serial_number: &SerialNumber) -> Option<Self> {
        Self::from_str(&serial_number.as_str()[12..])
    }

    fn from_str(model_number: &str) -> Option<Self> {
        Self::VARIANTS
            .iter()
            .find(|model| &model.as_ref()[10..] == model_number)
            .cloned()
    }

    pub async fn device_registry<B: ConnectionBackends + 'static>(
        &self,
        backends: B,
        database: Arc<OpenSCQ30Database>,
        is_demo: bool,
    ) -> crate::Result<Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>> {
        macro_rules! new_soundcore_device {
            ($module:ident) => {
                if is_demo {
                    Ok(Arc::new($module::demo_device_registry(database, *self)))
                } else {
                    Ok(Arc::new($module::device_registry::<B::Rfcomm>(
                        backends.rfcomm().await?,
                        database,
                        *self,
                    )))
                }
            };
        }
        match self {
            DeviceModel::SoundcoreA3027 | DeviceModel::SoundcoreA3030 => {
                new_soundcore_device!(a3027)
            }
            DeviceModel::SoundcoreA3028 => new_soundcore_device!(a3028),
            DeviceModel::SoundcoreA3029 => todo!(),
            DeviceModel::SoundcoreA3031 => new_soundcore_device!(a3031),
            DeviceModel::SoundcoreA3033 => new_soundcore_device!(a3033),
            DeviceModel::SoundcoreA3926 => new_soundcore_device!(a3926),
            DeviceModel::SoundcoreA3930 => new_soundcore_device!(a3930),
            DeviceModel::SoundcoreA3931 | DeviceModel::SoundcoreA3935 => {
                new_soundcore_device!(a3931)
            }
            DeviceModel::SoundcoreA3933 | DeviceModel::SoundcoreA3939 => {
                new_soundcore_device!(a3933)
            }
            DeviceModel::SoundcoreA3936 => new_soundcore_device!(a3936),
            DeviceModel::SoundcoreA3945 => new_soundcore_device!(a3945),
            DeviceModel::SoundcoreA3951 => new_soundcore_device!(a3951),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_serial_number() {
        assert!(DeviceModel::from_serial_number(&"0000000000003028".into()).is_some());
    }

    #[test]
    fn test_invalid_serial_number() {
        assert!(DeviceModel::from_serial_number(&"0000000000000000".into()).is_none());
    }
}
