use std::{
    borrow::Cow,
    str::FromStr,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use strum::IntoEnumIterator;
use tracing::{instrument, warn};

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::standard::{
        settings_manager::SettingHandler,
        structures::{EqualizerConfiguration, VolumeAdjustments, VolumeAdjustments2},
    },
    soundcore_device::device_model::DeviceModel,
    storage::OpenSCQ30Database,
};

use super::EqualizerSetting;

pub struct EqualizerSettingHandler {
    device_model: DeviceModel,
    database: Arc<OpenSCQ30Database>,
    custom_profiles: Mutex<Vec<(String, Vec<i16>)>>,
}

impl EqualizerSettingHandler {
    #[instrument(skip(database))]
    pub async fn new(database: Arc<OpenSCQ30Database>, device_model: DeviceModel) -> Self {
        let custom_profiles = database
            .fetch_all_equalizer_profiles(device_model)
            .await
            .unwrap_or_else(|err| {
                warn!("error fetching custom equalizer profiles, continuing without them: {err:?}");
                Vec::new()
            });
        Self {
            device_model,
            database,
            custom_profiles: Mutex::new(custom_profiles),
        }
    }

    async fn refresh(&self) -> crate::Result<()> {
        *self.custom_profiles.lock().unwrap() = self
            .database
            .fetch_all_equalizer_profiles(self.device_model)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<T> SettingHandler<T> for EqualizerSettingHandler
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration> + Send,
{
    fn settings(&self) -> Vec<SettingId<'static>> {
        EqualizerSetting::iter()
            .map(|variant| SettingId(Cow::Borrowed(variant.into())))
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<crate::api::settings::Setting> {
        let equalizer_configuration = state.as_ref();
        let volume_adjustments_2 =
            VolumeAdjustments2::from(equalizer_configuration.volume_adjustments().to_owned());
        let setting = EqualizerSetting::from_str(setting_id.0.as_ref()).ok()?;
        Some(match setting {
            EqualizerSetting::PresetProfile => Setting::optional_select_from_enum_all_variants(
                equalizer_configuration.preset_profile(),
            ),
            EqualizerSetting::CustomProfile => Setting::ModifiableSelect {
                setting: {
                    let custom_profiles = self.custom_profiles.lock().unwrap();
                    settings::Select {
                        options: custom_profiles
                            .iter()
                            .map(|(name, _)| name.to_owned().into())
                            .collect(),
                        localized_options: custom_profiles
                            .iter()
                            .map(|(name, _)| name.to_owned())
                            .collect(),
                    }
                },
                value: equalizer_configuration
                    .preset_profile()
                    .is_none()
                    .then(|| {
                        self.custom_profiles
                            .lock()
                            .unwrap()
                            .iter()
                            .find(|(_, v)| v == volume_adjustments_2.adjustments())
                            .map(|(name, _)| name.clone().into())
                    })
                    .flatten(),
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

    async fn set(&self, state: &mut T, setting_id: &SettingId, value: Value) -> crate::Result<()> {
        let equalizer_configuration = state.as_mut();
        let volume_adjustments_2 =
            VolumeAdjustments2::from(equalizer_configuration.volume_adjustments().to_owned());
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
            EqualizerSetting::CustomProfile => {
                if let Some(name) = value.try_as_optional_str()? {
                    if let Some(volume_adjustments) = self
                        .custom_profiles
                        .lock()
                        .unwrap()
                        .iter()
                        .find(|(n, _)| n == name)
                        .map(|(_, volume_adjustments)| volume_adjustments)
                    {
                        // Select existing profile
                        *state.as_mut() = EqualizerConfiguration::new_custom_profile(
                            VolumeAdjustments2::new(volume_adjustments.to_owned())
                                .unwrap()
                                .into(),
                        )
                    } else {
                        // Create new profile
                        self.database
                            .upsert_equalizer_profile(
                                self.device_model,
                                name.to_owned(),
                                volume_adjustments_2.adjustments().to_vec(),
                            )
                            .await?;
                        self.refresh().await?;
                    }
                } else {
                    // Delete
                    let maybe_name = self
                        .custom_profiles
                        .lock()
                        .unwrap()
                        .iter()
                        .find(|(_, volume_adjustments)| {
                            volume_adjustments == volume_adjustments_2.adjustments()
                        })
                        .map(|(name, _)| name)
                        .cloned();
                    if let Some(name) = maybe_name {
                        self.database
                            .delete_equalizer_profile(self.device_model, name.to_owned())
                            .await?;
                        self.refresh().await?;
                    }
                }
            }
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
