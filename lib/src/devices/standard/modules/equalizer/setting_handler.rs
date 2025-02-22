use std::{borrow::Cow, str::FromStr};

use strum::IntoEnumIterator;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::standard::{
        settings_manager::SettingHandler,
        structures::{EqualizerConfiguration, PresetEqualizerProfile, VolumeAdjustments},
    },
};

use super::EqualizerSetting;

#[derive(Default)]
pub struct EqualizerSettingHandler {}

impl<T> SettingHandler<T> for EqualizerSettingHandler
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration>,
{
    fn settings(&self) -> Vec<SettingId<'static>> {
        EqualizerSetting::iter()
            .map(|variant| SettingId(Cow::Borrowed(variant.into())))
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<crate::api::settings::Setting> {
        let equalizer_configuration = state.as_ref();
        let setting = EqualizerSetting::from_str(setting_id.0.as_ref()).ok()?;
        Some(match setting {
            EqualizerSetting::PresetProfile => Setting::OptionalSelect {
                setting: settings::Select {
                    options: PresetEqualizerProfile::iter()
                        .map(Into::into)
                        .map(Cow::Borrowed)
                        .collect(),
                },
                value: equalizer_configuration
                    .preset_profile()
                    .map(|preset| Cow::Borrowed(preset.into())),
            },
            EqualizerSetting::CustomProfile => Setting::OptionalSelect {
                setting: settings::Select {
                    options: Vec::new(),
                },
                value: None,
            },
            EqualizerSetting::VolumeAdjustments => Setting::Equalizer {
                setting: settings::Equalizer {
                    band_hz: Cow::Borrowed(&[100, 200, 400, 800, 1600, 3200, 6400, 12800]),
                    fraction_digits: 1,
                    min: -120,
                    max: 134,
                },
                values: equalizer_configuration
                    .volume_adjustments()
                    .adjustments()
                    .iter()
                    .map(|adj| (adj * 10f64) as i16)
                    .collect(),
            },
        })
    }

    fn set(&self, state: &mut T, setting_id: &SettingId, value: Value) -> crate::Result<()> {
        let equalizer_configuration = state.as_mut();
        let setting = EqualizerSetting::from_str(setting_id.0.as_ref())
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            EqualizerSetting::PresetProfile => {
                if let Some(preset) = value.try_as_optional_enum_variant()? {
                    *equalizer_configuration =
                        EqualizerConfiguration::new_from_preset_profile(preset)
                } else {
                    *equalizer_configuration = EqualizerConfiguration::new_custom_profile(
                        equalizer_configuration.volume_adjustments().to_owned(),
                    );
                }
            }
            EqualizerSetting::CustomProfile => todo!(),
            EqualizerSetting::VolumeAdjustments => {
                let volume_adjustments = value.try_as_i16_slice()?;
                *equalizer_configuration = EqualizerConfiguration::new_custom_profile(
                    VolumeAdjustments::new(
                        volume_adjustments.iter().map(|vol| *vol as f64 / 10f64),
                    )
                    .unwrap(),
                );
            }
        }
        Ok(())
    }
}
