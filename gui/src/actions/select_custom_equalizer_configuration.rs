use anyhow::Context;
use openscq30_lib::{
    api::device::DeviceRegistry,
    packets::structures::{EqualizerConfiguration, VolumeAdjustments},
};

use crate::{
    objects::CustomEqualizerProfileObject,
    settings::{Config, SettingsFile},
};

use super::{State, StateUpdate};

pub async fn select_custom_equalizer_configuration<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    custom_profile: &CustomEqualizerProfileObject,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    settings_file
        .get(|settings| {
            let profile = settings
                .custom_profiles()
                .get(&custom_profile.name())
                .ok_or_else(|| {
                    anyhow::anyhow!("custom profile does not exist: {}", custom_profile.name())
                })?;
            state
                .state_update_sender
                .send(StateUpdate::SetEqualizerConfiguration(
                    EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new(
                        profile.volume_adjustments(),
                    )),
                ))
                .map_err(|err| anyhow::anyhow!("{err}"))?;
            Ok(()) as anyhow::Result<()>
        })
        .context("unable to get equalizer config from settings file")??;
    Ok(())
}

#[cfg(test)]
mod tests {

    use openscq30_lib::packets::structures::{EqualizerConfiguration, VolumeAdjustments};

    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
        objects::CustomEqualizerProfileObject,
        settings::{Config, CustomEqualizerProfile, SettingsFile},
    };

    use super::select_custom_equalizer_configuration;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::<Config>::new(file.path().to_path_buf());
        let custom_profile = CustomEqualizerProfileObject::new(
            &"custom profile".to_string(),
            [1, 2, 3, 4, 5, 6, 7, 8],
        );
        settings_file
            .edit(|settings| {
                settings.set_custom_profile(
                    custom_profile.name(),
                    CustomEqualizerProfile::new(custom_profile.volume_adjustments()),
                );
            })
            .unwrap();

        select_custom_equalizer_configuration(&state, &settings_file, &custom_profile)
            .await
            .unwrap();

        let state_update = receiver.recv().await.unwrap();
        assert_eq!(
            StateUpdate::SetEqualizerConfiguration(
                EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new(
                    custom_profile.volume_adjustments()
                ))
                .into()
            ),
            state_update,
        );
    }
}
