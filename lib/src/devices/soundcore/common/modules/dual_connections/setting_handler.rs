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
        get_inner(dual_connections, setting_id)
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let dual_connections = state.get_mut();
        set_inner(dual_connections, setting_id, value)
    }
}

#[inline(never)]
fn get_inner(dual_connections: &DualConnections, setting_id: &SettingId) -> Option<Setting> {
    let setting: DualConnectionsSetting = (*setting_id).try_into().ok()?;
    Some(match setting {
        DualConnectionsSetting::DualConnections => Setting::Toggle {
            value: dual_connections.is_enabled,
        },
        DualConnectionsSetting::DualConnectionsDevices => Setting::MultiSelectWithRemove {
            setting: settings::Select {
                options: dual_connections
                    .devices
                    .iter()
                    .map(|device| Cow::from(device.mac_address.to_string()))
                    .collect(),
                localized_options: dual_connections
                    .devices
                    .iter()
                    .map(|device| device.name.clone())
                    .collect(),
            },
            values: dual_connections
                .devices
                .iter()
                .flat_map(|device| {
                    device
                        .is_connected
                        .then(|| Cow::from(device.mac_address.to_string()))
                })
                .collect(),
        },
    })
}

#[inline(never)]
fn set_inner(
    dual_connections: &mut DualConnections,
    setting_id: &SettingId,
    value: Value,
) -> SettingHandlerResult<()> {
    let setting: DualConnectionsSetting = (*setting_id)
        .try_into()
        .expect("already filtered to valid values only by SettingsManager");
    match setting {
        DualConnectionsSetting::DualConnections => {
            dual_connections.is_enabled = value.try_as_bool()?;
        }
        DualConnectionsSetting::DualConnectionsDevices => match value {
            Value::MultiSelectWithRemoveCommand(
                settings::MultiSelectWithRemoveCommand::Remove(mac_address),
            ) => {
                handle_remove_command(dual_connections, &mac_address);
            }
            Value::StringVec(mac_address_strings) => {
                handle_set_mac_addresses(dual_connections, mac_address_strings)?;
            }
            _ => {
                return Err(SettingHandlerError::ValueError(
                    settings::ValueError::WrongType {
                        expected: settings::ValueDiscriminants::StringVec,
                        actual: value,
                    },
                ));
            }
        },
    }
    Ok(())
}

#[tracing::instrument(level = "warn")]
fn handle_remove_command(dual_connections: &mut DualConnections, mac_address_str: &str) {
    let mac_address = match MacAddr6::from_str(mac_address_str) {
        Ok(mac_address) => mac_address,
        Err(err) => {
            tracing::warn!("failed to parse mac address {mac_address_str}: {err:?}");
            return;
        }
    };
    if let Some(index) = dual_connections
        .devices
        .iter()
        .position(|device| device.mac_address == mac_address)
    {
        dual_connections.devices.remove(index);
    }
}

fn handle_set_mac_addresses(
    dual_connections: &mut DualConnections,
    mac_address_strings: Vec<Cow<'static, str>>,
) -> SettingHandlerResult<()> {
    let desired_connections = mac_address_strings
        .into_iter()
        .map(|mac_address_str| MacAddr6::from_str(&mac_address_str))
        .collect::<Result<Vec<MacAddr6>, macaddr::ParseError>>()
        .map_err(|err| SettingHandlerError::Other(Box::new(err)))?;
    // Dual connections only allows for 2 connections, so don't allow connecting to more than that
    if desired_connections.len() <= 2 {
        for device in &mut dual_connections.devices {
            device.is_connected = desired_connections.contains(&device.mac_address);
        }
    }
    Ok(())
}
