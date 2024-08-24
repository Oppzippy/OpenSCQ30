use openscq30_lib::api::device::DeviceRegistry;

use crate::{
    actions,
    objects::GlibCustomEqualizerProfile,
    settings::{Config, CustomEqualizerProfile, SettingsFile},
};

use super::State;

pub fn import_custom_equalizer_profiles<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    custom_profiles: &[GlibCustomEqualizerProfile],
    overwrite: bool,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    settings_file.edit(|settings| {
        let profiles_for_insert = custom_profiles.iter().map(|profile| {
            (
                profile.name(),
                CustomEqualizerProfile::new(&profile.volume_adjustments()),
            )
        });
        settings.insert_custom_profiles(profiles_for_insert, overwrite);
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

    use super::import_custom_equalizer_profiles;

    fn insert_test_profiles(settings_file: &SettingsFile<Config>) {
        let profiles = [
            (
                "all zero".to_string(),
                CustomEqualizerProfile::new(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            ),
            (
                "all one".to_string(),
                CustomEqualizerProfile::new(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
            ),
            (
                "all two".to_string(),
                CustomEqualizerProfile::new(&[2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            ),
        ];
        settings_file
            .edit(move |config| {
                config.insert_custom_profiles(profiles, true);
            })
            .unwrap();
    }

    #[gtk::test]
    async fn it_updates_displayed_profiles() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::new(file.path().to_path_buf());

        let custom_profiles = &[GlibCustomEqualizerProfile::new(
            &"custom profile".to_string(),
            Arc::new([-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]),
        )];
        import_custom_equalizer_profiles(&state, &settings_file, custom_profiles, true).unwrap();

        let state_update = receiver.recv().await.unwrap();
        match state_update {
            StateUpdate::SetCustomEqualizerProfiles(_) => (),
            _ => panic!("set custom equalizer profiles was not sent"),
        }
    }

    #[gtk::test]
    async fn it_overwrites_existing_profiles() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::new(file.path().to_path_buf());
        insert_test_profiles(&settings_file);

        let custom_profiles = &[
            GlibCustomEqualizerProfile::new(
                &"custom profile".to_string(),
                Arc::new([-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]),
            ),
            GlibCustomEqualizerProfile::new(
                &"all zero".to_string(),
                Arc::new([-2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0]),
            ),
        ];
        import_custom_equalizer_profiles(&state, &settings_file, custom_profiles, true).unwrap();

        assert_eq!(
            4,
            settings_file
                .get(|settings| settings.custom_profiles().len())
                .unwrap()
        );
        assert_eq!(
            -2.0,
            settings_file
                .get(|settings| settings.custom_profiles()["all zero"].volume_adjustments()[0])
                .unwrap()
        );
    }

    #[gtk::test]
    async fn it_renames_when_profile_exists() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::new(file.path().to_path_buf());
        insert_test_profiles(&settings_file);

        let custom_profiles = &[GlibCustomEqualizerProfile::new(
            &"all zero".to_string(),
            Arc::new([-2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0]),
        )];
        import_custom_equalizer_profiles(&state, &settings_file, custom_profiles, false).unwrap();

        assert_eq!(
            0.0,
            settings_file
                .get(|settings| settings.custom_profiles()["all zero"].volume_adjustments()[0])
                .unwrap()
        );
        assert_eq!(
            -2.0,
            settings_file
                .get(|settings| settings.custom_profiles()["all zero (2)"].volume_adjustments()[0])
                .unwrap()
        );
    }

    #[gtk::test]
    async fn it_repeats_rename_until_available_name_is_found() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::new(file.path().to_path_buf());
        insert_test_profiles(&settings_file);
        settings_file
            .edit(|settings| {
                settings.set_custom_profile(
                    "all zero (2)".to_string(),
                    CustomEqualizerProfile::new(&[-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]),
                )
            })
            .unwrap();

        let custom_profiles = &[
            GlibCustomEqualizerProfile::new(
                &"all zero".to_string(),
                Arc::new([-2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0]),
            ),
            GlibCustomEqualizerProfile::new(
                &"all zero".to_string(),
                Arc::new([-3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0]),
            ),
        ];
        import_custom_equalizer_profiles(&state, &settings_file, custom_profiles, false).unwrap();

        assert_eq!(
            0.0,
            settings_file
                .get(|settings| settings.custom_profiles()["all zero"].volume_adjustments()[0])
                .unwrap()
        );
        assert_eq!(
            -1.0,
            settings_file
                .get(|settings| settings.custom_profiles()["all zero (2)"].volume_adjustments()[0])
                .unwrap()
        );
        assert_eq!(
            -2.0,
            settings_file
                .get(|settings| settings.custom_profiles()["all zero (3)"].volume_adjustments()[0])
                .unwrap()
        );
        assert_eq!(
            -3.0,
            settings_file
                .get(|settings| settings.custom_profiles()["all zero (4)"].volume_adjustments()[0])
                .unwrap()
        );
    }
}
