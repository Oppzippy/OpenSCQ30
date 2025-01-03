use anyhow::{anyhow, Context};
use openscq30_lib::api::device::DeviceRegistry;

use crate::{
    objects::GlibNamedQuickPresetValue,
    settings::{Config, SettingsFile},
};

use super::{State, StateUpdate};

pub fn refresh_quick_presets<T>(
    state: &State<T>,
    config: &SettingsFile<Config>,
    device_model: &str,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let quick_presets = config
        .get(|config| {
            config
                .quick_presets(device_model)
                .iter()
                .map(|(name, quick_preset)| GlibNamedQuickPresetValue {
                    name: name.as_str().into(),
                    quick_preset: quick_preset.to_owned(),
                })
                .collect()
        })
        .context("get quick presets from config")?;
    state
        .state_update_sender
        .send(StateUpdate::SetQuickPresets(quick_presets))
        .map_err(|err| anyhow!("{err:?}"))
}
