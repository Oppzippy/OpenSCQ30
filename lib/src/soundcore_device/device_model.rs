use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr, VariantArray};
use tokio::runtime::Handle;

use crate::{
    api::device::OpenSCQ30DeviceRegistry,
    devices::{
        a3027::{
            demo::DemoConnectionRegistry, device_profile::A3027DeviceRegistry,
            packets::A3027StateUpdatePacket,
        },
        standard::{packets::outbound::OutboundPacketBytesExt, structures::SerialNumber},
    },
    futures::{Futures, MaybeSend, MaybeSync},
};

use super::connection::new_connection_registry;

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

#[cfg(not(target_arch = "wasm32"))]
type MaybeSendDeviceRegistry = Arc<dyn OpenSCQ30DeviceRegistry + Send + Sync>;
#[cfg(target_arch = "wasm32")]
type MaybeSendDeviceRegistry = Arc<dyn OpenSCQ30DeviceRegistry>;

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

    pub async fn device_registry<F: Futures + 'static + MaybeSend + MaybeSync>(
        &self,
        runtime_handle: Option<Handle>,
        is_demo: bool,
    ) -> crate::Result<MaybeSendDeviceRegistry> {
        match self {
            DeviceModel::SoundcoreA3027 => Ok(if is_demo {
                Arc::new(A3027DeviceRegistry::<_, F>::new(
                    DemoConnectionRegistry::new(
                        "A3027".to_string(),
                        A3027StateUpdatePacket::default().bytes(),
                    ),
                    DeviceModel::SoundcoreA3027,
                ))
            } else {
                Arc::new(A3027DeviceRegistry::<_, F>::new(
                    new_connection_registry(runtime_handle).await?,
                    DeviceModel::SoundcoreA3027,
                ))
            }),
            DeviceModel::SoundcoreA3028 => todo!(),
            DeviceModel::SoundcoreA3029 => todo!(),
            DeviceModel::SoundcoreA3030 => todo!(),
            DeviceModel::SoundcoreA3031 => todo!(),
            DeviceModel::SoundcoreA3033 => todo!(),
            DeviceModel::SoundcoreA3926 => todo!(),
            DeviceModel::SoundcoreA3930 => todo!(),
            DeviceModel::SoundcoreA3931 => todo!(),
            DeviceModel::SoundcoreA3933 => todo!(),
            DeviceModel::SoundcoreA3936 => todo!(),
            DeviceModel::SoundcoreA3945 => todo!(),
            DeviceModel::SoundcoreA3951 => todo!(),
            DeviceModel::SoundcoreA3939 => todo!(),
            DeviceModel::SoundcoreA3935 => todo!(),
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
