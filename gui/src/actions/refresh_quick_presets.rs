use anyhow::anyhow;
use openscq30_lib::api::device::DeviceRegistry;
use uuid::Uuid;

use crate::{
    objects::NamedQuickPreset,
    settings::{Config, SettingsFile},
};

use super::{State, StateUpdate};

pub fn refresh_quick_presets<T>(
    state: &State<T>,
    config: &SettingsFile<Config>,
    device_service_uuid: Uuid,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let quick_presets = config.get(|config| {
        config
            .quick_presets(device_service_uuid)
            .iter()
            .map(|(name, quick_preset)| NamedQuickPreset {
                name: name.as_str().into(),
                quick_preset: quick_preset.to_owned(),
            })
            .collect()
    })?;
    state
        .state_update_sender
        .send(StateUpdate::SetQuickPresets(quick_presets))
        .map_err(|err| anyhow!("{err:?}"))
}
