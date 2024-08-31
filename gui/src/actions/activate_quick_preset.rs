use anyhow::Context;
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    devices::standard::{
        state::DeviceState,
        structures::{EqualizerConfiguration, SoundModes, VolumeAdjustments},
    },
};

use crate::settings::{Config, PresetOrCustomEqualizerProfile, QuickPreset, SettingsFile};

use super::State;

pub async fn activate_quick_preset<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    quick_preset: &QuickPreset,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    if let Some(device) = state.selected_device() {
        let device_state = device.state().await;
        set_sound_modes_from_quick_preset(device.as_ref(), &device_state, quick_preset).await?;
        set_equalizer_configuration_from_quick_preset(device.as_ref(), settings_file, quick_preset)
            .await?;
    }
    Ok(())
}

async fn set_sound_modes_from_quick_preset(
    device: &impl Device,
    device_state: &DeviceState,
    quick_preset: &QuickPreset,
) -> anyhow::Result<()> {
    if let Some(sound_modes) = device_state.sound_modes {
        let new_sound_modes = SoundModes {
            ambient_sound_mode: quick_preset
                .ambient_sound_mode
                .unwrap_or(sound_modes.ambient_sound_mode),
            transparency_mode: quick_preset
                .transparency_mode
                .unwrap_or(sound_modes.transparency_mode),
            noise_canceling_mode: quick_preset
                .noise_canceling_mode
                .unwrap_or(sound_modes.noise_canceling_mode),
            custom_noise_canceling: quick_preset
                .custom_noise_canceling
                .unwrap_or(sound_modes.custom_noise_canceling),
        };
        device
            .set_sound_modes(new_sound_modes)
            .await
            .context("set sound modes")?;
    }
    Ok(())
}

async fn set_equalizer_configuration_from_quick_preset(
    device: &impl Device,
    settings_file: &SettingsFile<Config>,
    quick_preset: &QuickPreset,
) -> anyhow::Result<()> {
    let new_equalizer_configuration = match &quick_preset.equalizer_profile {
        Some(PresetOrCustomEqualizerProfile::Preset(profile)) => {
            Some(EqualizerConfiguration::new_from_preset_profile(*profile))
        }
        Some(PresetOrCustomEqualizerProfile::Custom(custom_profile_name)) => {
            let profile = settings_file
                .get(|config| {
                    config
                        .custom_profiles()
                        .get(custom_profile_name.as_ref())
                        .cloned()
                })
                .context("get custom eq profile from config")?;
            if let Some(profile) = profile {
                let volume_adjustments =
                    VolumeAdjustments::new(profile.volume_adjustments().iter().cloned())
                        .context("parse volume adjustments of custom eq profile")?;
                Some(EqualizerConfiguration::new_custom_profile(
                    volume_adjustments,
                ))
            } else {
                None
            }
        }
        None => None,
    };

    if let Some(new_equalizer_configuration) = new_equalizer_configuration {
        device
            .set_equalizer_configuration(new_equalizer_configuration)
            .await
            .context("set equalizer configuration")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use openscq30_lib::devices::standard::{
        state::DeviceState,
        structures::{
            AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
            PresetEqualizerProfile, SoundModes, TransparencyMode, VolumeAdjustments,
        },
    };
    use uuid::Uuid;

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
        settings::{
            Config, CustomEqualizerProfile, PresetOrCustomEqualizerProfile, QuickPreset,
            SettingsFile,
        },
    };

    use super::activate_quick_preset;

    #[gtk::test]
    async fn test_set_ambient_sound_mode() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _sender) = State::new(registry);

        let mut device = MockDevice::new();
        device.expect_service_uuid().return_const(Uuid::default());
        device.expect_state().return_const(DeviceState {
            sound_modes: Some(SoundModes {
                ambient_sound_mode: AmbientSoundMode::Normal,
                transparency_mode: TransparencyMode::FullyTransparent,
                noise_canceling_mode: NoiseCancelingMode::Transport,
                custom_noise_canceling: CustomNoiseCanceling::new(3),
            }),
            ..Default::default()
        });
        device
            .expect_set_sound_modes()
            .withf(|sound_modes| {
                sound_modes
                    == &SoundModes {
                        ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
                        transparency_mode: TransparencyMode::FullyTransparent,
                        noise_canceling_mode: NoiseCancelingMode::Transport,
                        custom_noise_canceling: CustomNoiseCanceling::new(3),
                    }
            })
            .once()
            .returning(|_| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(device));

        let dir = tempfile::tempdir().unwrap();
        let settings_file = SettingsFile::new(dir.path().join("config.toml"));

        let quick_preset = QuickPreset {
            ambient_sound_mode: Some(AmbientSoundMode::NoiseCanceling),
            ..Default::default()
        };

        activate_quick_preset(&state, &settings_file, &quick_preset)
            .await
            .unwrap();
    }

    #[gtk::test]
    async fn test_set_preset_equalizer_profile() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _sender) = State::new(registry);

        let mut device = MockDevice::new();
        device.expect_service_uuid().return_const(Uuid::default());
        device.expect_state().return_const(DeviceState::default());
        device
            .expect_set_equalizer_configuration()
            .withf(|equalizer_configuration| {
                equalizer_configuration
                    == &EqualizerConfiguration::new_from_preset_profile(
                        PresetEqualizerProfile::Acoustic,
                    )
            })
            .once()
            .returning(|_| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(device));

        let dir = tempfile::tempdir().unwrap();
        let settings_file = SettingsFile::new(dir.path().join("config.toml"));

        let quick_preset = QuickPreset {
            equalizer_profile: Some(PresetOrCustomEqualizerProfile::Preset(
                PresetEqualizerProfile::Acoustic,
            )),
            ..Default::default()
        };

        activate_quick_preset(&state, &settings_file, &quick_preset)
            .await
            .unwrap();
    }

    #[gtk::test]
    async fn test_set_custom_equalizer_profile() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _sender) = State::new(registry);

        const VOLUME_ADJUSTMENTS: [f64; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let mut device = MockDevice::new();
        device.expect_service_uuid().return_const(Uuid::default());
        device.expect_state().return_const(DeviceState::default());
        device
            .expect_set_equalizer_configuration()
            .withf(move |equalizer_configuration| {
                equalizer_configuration
                    == &EqualizerConfiguration::new_custom_profile(
                        VolumeAdjustments::new(VOLUME_ADJUSTMENTS).unwrap(),
                    )
            })
            .once()
            .returning(|_| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(device));

        let dir = tempfile::tempdir().unwrap();
        let settings_file = SettingsFile::new(dir.path().join("config.toml"));
        settings_file
            .edit(|config: &mut Config| {
                config.set_custom_profile(
                    "test profile".into(),
                    CustomEqualizerProfile::new(&VOLUME_ADJUSTMENTS),
                );
            })
            .unwrap();

        let quick_preset = QuickPreset {
            equalizer_profile: Some(PresetOrCustomEqualizerProfile::Custom(
                "test profile".into(),
            )),
            ..Default::default()
        };

        activate_quick_preset(&state, &settings_file, &quick_preset)
            .await
            .unwrap();
    }

    #[gtk::test]
    async fn test_set_deleted_custom_equalizer_profile() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _sender) = State::new(registry);

        let mut device = MockDevice::new();
        device.expect_service_uuid().return_const(Uuid::default());
        device.expect_state().return_const(DeviceState::default());
        device.expect_set_equalizer_configuration().never();
        *state.selected_device.borrow_mut() = Some(Rc::new(device));

        let dir = tempfile::tempdir().unwrap();
        let settings_file = SettingsFile::new(dir.path().join("config.toml"));

        let quick_preset = QuickPreset {
            equalizer_profile: Some(PresetOrCustomEqualizerProfile::Custom(
                "deleted profile".into(),
            )),
            ..Default::default()
        };

        activate_quick_preset(&state, &settings_file, &quick_preset)
            .await
            .unwrap();
    }
}
