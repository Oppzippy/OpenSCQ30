use std::sync::Arc;

use anyhow::Context;
use openscq30_lib::{
    api::device::DeviceRegistry,
    devices::standard::structures::{EqualizerConfiguration, VolumeAdjustments},
};

use crate::{
    objects::GlibCustomEqualizerProfile,
    settings::{Config, SettingsFile},
};

use super::{set_equalizer_configuration, State, StateUpdate};

pub async fn select_custom_equalizer_configuration<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    custom_profile: &GlibCustomEqualizerProfile,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let volume_adjustments = settings_file
        .get(|settings| {
            let profile = settings
                .custom_profiles()
                .get(&custom_profile.name())
                .ok_or_else(|| {
                    anyhow::anyhow!("custom profile does not exist: {}", custom_profile.name())
                })?;
            Ok(profile.volume_adjustments()) as anyhow::Result<Arc<[f64]>>
        })
        .context("unable to get equalizer config from settings file")??;

    let volume_adjustments = VolumeAdjustments::new(volume_adjustments.iter().cloned())?;
    let equalizer_configuration = EqualizerConfiguration::new_custom_profile(volume_adjustments);

    state
        .state_update_sender
        .send(StateUpdate::SetEqualizerConfiguration(
            equalizer_configuration.to_owned(),
        ))
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    set_equalizer_configuration(state, equalizer_configuration).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::{rc::Rc, sync::Arc};

    use mockall::predicate;
    use openscq30_lib::devices::standard::structures::{EqualizerConfiguration, VolumeAdjustments};

    use crate::{
        actions::{State, StateUpdate},
        mock::{MockDevice, MockDeviceRegistry},
        objects::GlibCustomEqualizerProfile,
        settings::{Config, CustomEqualizerProfile, SettingsFile},
    };

    use super::select_custom_equalizer_configuration;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);

        let dir = tempfile::tempdir().unwrap();
        let settings_file = SettingsFile::<Config>::new(dir.path().join("config.toml"));
        let custom_profile = GlibCustomEqualizerProfile::new(
            &"custom profile".to_string(),
            Arc::new([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]),
        );
        let lib_custom_profile = EqualizerConfiguration::new_custom_profile(
            VolumeAdjustments::new(custom_profile.volume_adjustments().iter().cloned()).unwrap(),
        );
        settings_file
            .edit(|settings| {
                settings.set_custom_profile(
                    custom_profile.name(),
                    CustomEqualizerProfile::new(&custom_profile.volume_adjustments()),
                );
            })
            .unwrap();

        let mut selected_device = MockDevice::new();
        selected_device
            .expect_set_equalizer_configuration()
            .once()
            .with(predicate::eq(lib_custom_profile.to_owned()))
            .return_once(|_ambient_sound_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        select_custom_equalizer_configuration(&state, &settings_file, &custom_profile)
            .await
            .unwrap();

        let state_update = receiver.recv().await.unwrap();
        assert_eq!(
            StateUpdate::SetEqualizerConfiguration(lib_custom_profile),
            state_update,
        );
    }
}
