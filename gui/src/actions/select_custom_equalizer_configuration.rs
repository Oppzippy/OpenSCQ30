use openscq30_lib::{
    api::device::DeviceRegistry,
    packets::structures::{EqualizerBandOffsets, EqualizerConfiguration},
};

use crate::{objects::EqualizerCustomProfileObject, settings::SettingsFile};

use super::{State, StateUpdate};

pub fn select_custom_equalizer_configuration<T>(
    state: &State<T>,
    settings_file: &SettingsFile,
    custom_profile: &EqualizerCustomProfileObject,
) where
    T: DeviceRegistry + Send + Sync + 'static,
{
    let result = settings_file.get(|settings| {
        match settings.custom_profiles().get(&custom_profile.name()) {
            Some(profile) => {
                state
                    .state_update_sender
                    .send(StateUpdate::SetEqualizerConfiguration(
                        EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new(
                            profile.volume_offsets(),
                        )),
                    ))
                    .unwrap();
            }
            None => {
                tracing::warn!("custom profile does not exist: {}", custom_profile.name());
            }
        }
    });
    if let Err(err) = result {
        tracing::warn!("unable to get settings file: {:?}", err);
    }
}

#[cfg(test)]
mod tests {

    use openscq30_lib::packets::structures::{EqualizerBandOffsets, EqualizerConfiguration};

    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
        objects::EqualizerCustomProfileObject,
        settings::{EqualizerCustomProfile, SettingsFile},
    };

    use super::select_custom_equalizer_configuration;

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
        settings_file
            .edit(|settings| {
                settings.set_custom_profile(
                    custom_profile.name(),
                    EqualizerCustomProfile::new(custom_profile.volume_offsets()),
                );
            })
            .unwrap();
        select_custom_equalizer_configuration(&state, &settings_file, &custom_profile);

        let state_update = receiver.recv().await.unwrap();
        assert_eq!(
            StateUpdate::SetEqualizerConfiguration(EqualizerConfiguration::new_custom_profile(
                EqualizerBandOffsets::new(custom_profile.volume_offsets())
            )),
            state_update,
        );
    }
}