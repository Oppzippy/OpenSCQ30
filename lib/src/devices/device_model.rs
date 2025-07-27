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
            Self::SoundcoreA3004 => new_soundcore_device!(soundcore::a3004),
            Self::SoundcoreA3027 | Self::SoundcoreA3030 => {
                new_soundcore_device!(soundcore::a3027)
            }
            Self::SoundcoreA3028 | Self::SoundcoreA3029 => {
                new_soundcore_device!(soundcore::a3028)
            }
            Self::SoundcoreA3031 => new_soundcore_device!(soundcore::a3031),
            Self::SoundcoreA3033 => new_soundcore_device!(soundcore::a3033),
            Self::SoundcoreA3926 => new_soundcore_device!(soundcore::a3926),
            Self::SoundcoreA3930 => new_soundcore_device!(soundcore::a3930),
            Self::SoundcoreA3931 | Self::SoundcoreA3935 => {
                new_soundcore_device!(soundcore::a3931)
            }
            Self::SoundcoreA3933 | Self::SoundcoreA3939 => {
                new_soundcore_device!(soundcore::a3933)
            }
            Self::SoundcoreA3936 => new_soundcore_device!(soundcore::a3936),
            Self::SoundcoreA3945 => new_soundcore_device!(soundcore::a3945),
            Self::SoundcoreA3951 => new_soundcore_device!(soundcore::a3951),
            Self::SoundcoreA3959 => new_soundcore_device!(soundcore::a3959),
            Self::SoundcoreDevelopment => new_soundcore_device!(soundcore::development),
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
            Self::SoundcoreA3004 => new_soundcore_device!(soundcore::a3004),

            Self::SoundcoreA3027 | Self::SoundcoreA3030 => {
                new_soundcore_device!(soundcore::a3027)
            }
            Self::SoundcoreA3028 | Self::SoundcoreA3029 => {
                new_soundcore_device!(soundcore::a3028)
            }
            Self::SoundcoreA3031 => new_soundcore_device!(soundcore::a3031),
            Self::SoundcoreA3033 => new_soundcore_device!(soundcore::a3033),
            Self::SoundcoreA3926 => new_soundcore_device!(soundcore::a3926),
            Self::SoundcoreA3930 => new_soundcore_device!(soundcore::a3930),
            Self::SoundcoreA3931 | Self::SoundcoreA3935 => {
                new_soundcore_device!(soundcore::a3931)
            }
            Self::SoundcoreA3933 | Self::SoundcoreA3939 => {
                new_soundcore_device!(soundcore::a3933)
            }
            Self::SoundcoreA3936 => new_soundcore_device!(soundcore::a3936),
            Self::SoundcoreA3945 => new_soundcore_device!(soundcore::a3945),
            Self::SoundcoreA3951 => new_soundcore_device!(soundcore::a3951),
            Self::SoundcoreA3959 => new_soundcore_device!(soundcore::a3959),
            Self::SoundcoreDevelopment => new_soundcore_device!(soundcore::development),
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

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::api::settings::{Setting, Value};

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn test_set_all_settings() {
        let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());
        for model in DeviceModel::iter() {
            let device_registry = model
                .demo_device_registry(database.clone())
                .await
                .expect(&format!("{model} device registry"));
            let device = device_registry
                .connect(MacAddr6::nil())
                .await
                .expect(&format!("{model} device"));
            for category_id in device.categories() {
                for setting_id in device.settings_in_category(&category_id) {
                    let setting = device.setting(&setting_id).expect(&format!(
                        "getting setting {model} -> {category_id} -> {setting_id}"
                    ));
                    let value = match setting {
                        Setting::Toggle { value } => Some(Value::from(!value)),
                        Setting::I32Range { value, .. } => Some(value.into()),
                        Setting::Select { value, .. } => Some(value.into()),
                        Setting::OptionalSelect { value, .. } => Some(value.into()),
                        Setting::MultiSelect { setting, .. } => Some(setting.options.into()),
                        Setting::Equalizer { value, .. } => Some(value.into()),
                        Setting::ModifiableSelect { .. }
                        | Setting::Information { .. }
                        | Setting::ImportString { .. } => None,
                    };
                    if let Some(value) = value {
                        device
                            .set_setting_values(vec![(setting_id, value)])
                            .await
                            .expect(&format!("setting {model} -> {category_id} -> {setting_id}"));
                    }
                }
            }
        }
    }
}
