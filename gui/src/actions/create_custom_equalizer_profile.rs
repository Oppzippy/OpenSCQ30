use openscq30_lib::api::device::DeviceRegistry;

use crate::{
    objects::EqualizerCustomProfileObject,
    settings::{EqualizerCustomProfile, SettingsFile},
};

use super::{State, StateUpdate};

pub fn create_custom_equalizer_profile<T>(
    state: &State<T>,
    settings_file: &SettingsFile,
    custom_profile: &EqualizerCustomProfileObject,
) where
    T: DeviceRegistry + Send + Sync + 'static,
{
    settings_file
        .edit(|settings| {
            settings.set_custom_profile(
                custom_profile.name(),
                EqualizerCustomProfile::new(custom_profile.volume_offsets()),
            );
        })
        .unwrap();
    settings_file
        .get(|settings| {
            state
                .state_update_sender
                .send(StateUpdate::SetEqualizerCustomProfiles(
                    settings
                        .custom_profiles()
                        .iter()
                        .map(|(name, profile)| {
                            EqualizerCustomProfileObject::new(name, profile.volume_offsets())
                        })
                        .collect(),
                ))
                .unwrap();
        })
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
        objects::EqualizerCustomProfileObject,
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
        let custom_profile = EqualizerCustomProfileObject::new(
            &"custom profile".to_string(),
            [1, 2, 3, 4, 5, 6, 7, 8],
        );
        create_custom_equalizer_profile(&state, &settings_file, &custom_profile);

        let state_update = receiver.recv().await.unwrap();
        if let StateUpdate::SetEqualizerCustomProfiles(profiles) = state_update {
            let profile = profiles.first().unwrap();
            assert_eq!(custom_profile.name(), profile.name());
            assert_eq!(custom_profile.volume_offsets(), profile.volume_offsets());
        } else {
            panic!("StateUpdate was not SetEqualizerCustomProfiles");
        }
        assert_eq!(
            1,
            settings_file
                .get(|settings| settings.custom_profiles().len())
                .unwrap()
        );
    }
}
