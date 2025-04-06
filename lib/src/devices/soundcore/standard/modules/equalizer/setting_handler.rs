use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use strum::IntoEnumIterator;
use tracing::{instrument, warn};

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::{
        DeviceModel,
        soundcore::standard::{
            settings_manager::SettingHandler,
            structures::{EqualizerConfiguration, VolumeAdjustments},
        },
    },
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

    fn values_to_volume_adjustments(
        &self,
        values: &[i16],
        existing_volume_adjustments: &[VolumeAdjustments],
    ) -> Vec<VolumeAdjustments> {
        // Some devices have extra bands, but those aren't exposed to the user, so I have no idea what they're for
        // We can just add back in whatever was there before (we're only showing the user the first 8 bands)
        existing_volume_adjustments.iter().map(|volume_adjustments| {
            VolumeAdjustments::new(
                values
                .iter()
                .take(8)
                .chain(
                    volume_adjustments
                        .adjustments()
                        .iter()
                        .skip(8),
                )
                .cloned()
                .collect(),
            ).expect("we have control over the number of values in these vecs, so it should always be a valid number")
        }).collect()
    }
}

#[async_trait]
impl<T> SettingHandler<T> for EqualizerSettingHandler
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        EqualizerSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<crate::api::settings::Setting> {
        let equalizer_configuration = state.as_ref();
        let setting = setting_id.try_into().ok()?;
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
                            .find(|(_, v)| {
                                v == equalizer_configuration
                                    .volume_adjustments_channel_1()
                                    .adjustments()
                            })
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
                    .volume_adjustments_channel_1()
                    .adjustments()
                    .to_vec(),
            },
        })
    }

    async fn set(&self, state: &mut T, setting_id: &SettingId, value: Value) -> crate::Result<()> {
        let equalizer_configuration = state.as_mut();
        let setting = setting_id
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            EqualizerSetting::PresetProfile => {
                if let Some(preset) = value.try_as_optional_enum_variant()? {
                    *equalizer_configuration = EqualizerConfiguration::new_from_preset_profile(
                        equalizer_configuration.channels(),
                        preset,
                        equalizer_configuration
                            .volume_adjustments()
                            .iter()
                            .map(|v| v.adjustments().iter().cloned().skip(8).collect())
                            .collect(),
                    )
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
                        // Activate existing profile
                        *state.as_mut() = EqualizerConfiguration::new_custom_profile(
                            self.values_to_volume_adjustments(
                                volume_adjustments,
                                &equalizer_configuration.volume_adjustments(),
                            ),
                        )
                    } else {
                        // Create new profile
                        self.database
                            .upsert_equalizer_profile(
                                self.device_model,
                                name.to_owned(),
                                equalizer_configuration
                                    .volume_adjustments_channel_1()
                                    .adjustments()
                                    .to_vec(),
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
                            volume_adjustments
                                == equalizer_configuration
                                    .volume_adjustments_channel_1()
                                    .adjustments()
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
                    self.values_to_volume_adjustments(
                        volume_adjustments,
                        equalizer_configuration.volume_adjustments(),
                    )
                    .into(),
                );
            }
        }
        Ok(())
    }
}
