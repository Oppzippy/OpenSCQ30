use anyhow::Context;
use openscq30_lib::api::device::DeviceRegistry;

use crate::{
    actions,
    objects::GlibCustomEqualizerProfile,
    settings::{Config, CustomEqualizerProfile, SettingsFile},
};

use super::State;

pub fn create_custom_equalizer_profile<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    custom_profile: &GlibCustomEqualizerProfile,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    insert_custom_profile(settings_file, custom_profile)?;
    actions::refresh_custom_equalizer_profiles(&state.state_update_sender, settings_file)?;
    Ok(())
}

fn insert_custom_profile(
    settings_file: &SettingsFile<Config>,
    custom_profile: &GlibCustomEqualizerProfile,
) -> anyhow::Result<()> {
    settings_file
        .edit(|settings| {
            settings.set_custom_profile(
                custom_profile.name(),
                CustomEqualizerProfile::new(&custom_profile.volume_adjustments()),
            );
        })
        .context("insert custom profile")
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
        objects::GlibCustomEqualizerProfile,
        settings::SettingsFile,
    };

    use super::create_custom_equalizer_profile;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);

        let dir = tempfile::tempdir().unwrap();
        let settings_file = SettingsFile::new(dir.path().join("config.toml"));
        let custom_profile = GlibCustomEqualizerProfile::new(
            "custom profile",
            Arc::new([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]),
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
