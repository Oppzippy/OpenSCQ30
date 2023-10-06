use anyhow::Context;
use openscq30_lib::api::device::{Device, DeviceRegistry};

use crate::{
    objects::NamedQuickPreset,
    settings::{Config, SettingsFile},
};

use super::{State, StateUpdate};

pub fn delete_quick_preset<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    quick_preset_name: &str,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let Some(device) = state.selected_device() else {
        anyhow::bail!("cannot delete quick preset while not connected to a device");
    };
    let device_service_uuid = device.service_uuid();

    settings_file
        .edit(|settings| {
            settings.remove_quick_preset(device_service_uuid, quick_preset_name);
        })
        .context("edit settings")?;
    settings_file
        .get(|settings| {
            state
                .state_update_sender
                .send(StateUpdate::SetQuickPresets(
                    settings
                        .quick_presets(device_service_uuid)
                        .iter()
                        .map(|(name, quick_preset)| NamedQuickPreset {
                            name: name.to_owned(),
                            quick_preset: quick_preset.to_owned(),
                        })
                        .collect(),
                ))
                .unwrap();
        })
        .context("get updated settings")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use uuid::Uuid;

    use crate::{
        actions::{State, StateUpdate},
        mock::{MockDevice, MockDeviceRegistry},
        objects::NamedQuickPreset,
        settings::{Config, QuickPreset, SettingsFile},
    };

    use super::delete_quick_preset;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);
        let mut device = MockDevice::new();
        device.expect_service_uuid().return_const(Uuid::default());
        *state.selected_device.borrow_mut() = Some(Rc::new(device));

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::new(file.path().to_path_buf());
        settings_file
            .edit(|config: &mut Config| {
                config.set_quick_preset(Uuid::default(), "test", QuickPreset::default());
            })
            .unwrap();
        delete_quick_preset(&state, &settings_file, "test").unwrap();

        let state_update = receiver.recv().await.unwrap();
        if let StateUpdate::SetQuickPresets(quick_presets) = state_update {
            assert_eq!(Vec::<NamedQuickPreset>::new(), quick_presets);
        } else {
            panic!("StateUpdate was not RefreshQuickPresets: {state_update:?}");
        }
    }
}
