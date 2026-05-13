use std::{borrow::Cow, str::FromStr};

use async_trait::async_trait;
use macaddr::MacAddr6;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::DualConnections,
    },
    settings,
};

use super::DualConnectionsSetting;

pub struct DualConnectionsSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for DualConnectionsSettingHandler
where
    T: Has<DualConnections> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        DualConnectionsSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let dual_connections = state.get();
        let setting: DualConnectionsSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            DualConnectionsSetting::DualConnections => Setting::Toggle {
                value: dual_connections.is_enabled,
            },
            DualConnectionsSetting::DualConnectionsDevices => Setting::MultiSelect {
                setting: settings::Select {
                    options: dual_connections
                        .devices
                        .iter()
                        .flatten()
                        .map(|device| Cow::from(device.mac_address.to_string()))
                        .collect(),
                    localized_options: dual_connections
                        .devices
                        .iter()
                        .flatten()
                        .map(|device| device.name.clone())
                        .collect(),
                },
                values: dual_connections
                    .devices
                    .iter()
                    .flatten()
                    .flat_map(|device| {
                        device
                            .is_connected
                            .then(|| Cow::from(device.mac_address.to_string()))
                    })
                    .collect(),
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let dual_connections = state.get_mut();
        let setting: DualConnectionsSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            DualConnectionsSetting::DualConnections => {
                dual_connections.is_enabled = value.try_as_bool()?;
            }
            DualConnectionsSetting::DualConnectionsDevices => {
                let desired_connections = value
                    .try_into_string_vec()?
                    .into_iter()
                    .map(|mac_address_str| MacAddr6::from_str(&mac_address_str))
                    .collect::<Result<Vec<MacAddr6>, macaddr::ParseError>>()
                    .map_err(|err| SettingHandlerError::Other(Box::new(err)))?;
                for device in &mut dual_connections.devices.iter_mut().flatten() {
                    device.is_connected = desired_connections.contains(&device.mac_address);
                }
            }
        }
        Ok(())
    }
}
