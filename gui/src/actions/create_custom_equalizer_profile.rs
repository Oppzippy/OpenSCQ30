use openscq30_lib::api::device::DeviceRegistry;

use crate::{
    objects::CustomEqualizerProfileObject,
    settings::{Config, CustomEqualizerProfile, SettingsFile},
};

use super::{State, StateUpdate};

pub fn create_custom_equalizer_profile<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    custom_profile: &CustomEqualizerProfileObject,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    settings_file.edit(|settings| {
        settings.set_custom_profile(
            custom_profile.name(),
            CustomEqualizerProfile::new(custom_profile.volume_adjustments()),
        );
    })?;
    settings_file.get(|settings| {
        state
            .state_update_sender
            .send(StateUpdate::SetCustomEqualizerProfiles(
                settings
                    .custom_profiles()
                    .iter()
                    .map(|(name, profile)| {
                        CustomEqualizerProfileObject::new(name, profile.volume_adjustments())
                    })
                    .collect(),
            ))
            .unwrap();
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
        objects::CustomEqualizerProfileObject,
        settings::SettingsFile,
    };

    use super::create_custom_equalizer_profile;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::new(file.path().to_path_buf());
        let custom_profile = CustomEqualizerProfileObject::new(
            &"custom profile".to_string(),
            [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
        );
        create_custom_equalizer_profile(&state, &settings_file, &custom_profile).unwrap();

        let state_update = receiver.recv().await.unwrap();
        if let StateUpdate::SetCustomEqualizerProfiles(profiles) = state_update {
            let profile = profiles.first().unwrap();
            assert_eq!(custom_profile.name(), profile.name());
            assert_eq!(
                custom_profile.volume_adjustments(),
                profile.volume_adjustments()
            );
        } else {
            panic!("StateUpdate was not SetCustomEqualizerProfiles");
        }
        assert_eq!(
            1,
            settings_file
                .get(|settings| settings.custom_profiles().len())
                .unwrap()
        );
    }
}
