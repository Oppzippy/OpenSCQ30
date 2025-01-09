use openscq30_lib::api::device::DeviceRegistry;

use crate::{
    actions,
    objects::GlibCustomEqualizerProfile,
    settings::{Config, SettingsFile},
};

use super::State;

pub fn delete_custom_equalizer_profile<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    custom_profile: &GlibCustomEqualizerProfile,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    settings_file.edit(|settings| {
        settings.remove_custom_profile(&custom_profile.name());
    })?;
    actions::refresh_custom_equalizer_profiles(&state.state_update_sender, settings_file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
        objects::GlibCustomEqualizerProfile,
        settings::{Config, CustomEqualizerProfile, SettingsFile},
    };

    use super::delete_custom_equalizer_profile;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);

        let dir = tempfile::tempdir().unwrap();
        let settings_file = SettingsFile::<Config>::new(dir.path().join("config.toml"));
        let custom_profile = GlibCustomEqualizerProfile::new(
            "custom profile",
            Arc::new([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]),
        );
        settings_file
            .edit(|settings| {
                settings.set_custom_profile(
                    custom_profile.name(),
                    CustomEqualizerProfile::new(&custom_profile.volume_adjustments()),
                );
            })
            .unwrap();
        delete_custom_equalizer_profile(&state, &settings_file, &custom_profile).unwrap();

        let state_update = receiver.recv().await.unwrap();
        assert_eq!(
            StateUpdate::SetCustomEqualizerProfiles(Vec::new()),
            state_update,
        );
        assert_eq!(
            0,
            settings_file
                .get(|settings| settings.custom_profiles().len())
                .unwrap()
        );
    }
}
