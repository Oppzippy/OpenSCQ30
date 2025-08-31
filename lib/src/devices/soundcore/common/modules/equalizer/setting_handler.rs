use std::{array, borrow::Cow, sync::Arc};

use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;
use tokio::sync::watch;
use tracing::instrument;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::equalizer::custom_equalizer_profile_store::CustomEqualizerProfileStore,
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::{EqualizerConfiguration, VolumeAdjustments},
    },
};

use super::EqualizerSetting;

pub struct EqualizerSettingHandler<const C: usize, const B: usize> {
    profile_store: Arc<CustomEqualizerProfileStore>,
    custom_profiles_receiver: watch::Receiver<Vec<(String, Vec<i16>)>>,
}

impl<const C: usize, const B: usize> EqualizerSettingHandler<C, B> {
    #[instrument(skip(profile_store))]
    pub async fn new(profile_store: Arc<CustomEqualizerProfileStore>) -> Self {
        Self {
            custom_profiles_receiver: profile_store.subscribe(),
            profile_store,
        }
    }

    fn values_to_volume_adjustments(
        &self,
        values: &[i16],
        existing_volume_adjustments: &[VolumeAdjustments<B>; C],
    ) -> [VolumeAdjustments<B>; C] {
        // Some devices have extra bands, but those aren't exposed to the user, so I have no idea what they're for
        // We can just add back in whatever was there before (we're only showing the user the first 8 bands)
        array::from_fn(|i| {
            VolumeAdjustments::new(array::from_fn(|j| {
                if j < values.len() {
                    values[j]
                } else {
                    existing_volume_adjustments[i].adjustments()[j - values.len()]
                }
            }))
        })
    }
}

#[async_trait]
impl<T, const C: usize, const B: usize> SettingHandler<T> for EqualizerSettingHandler<C, B>
where
    T: Has<EqualizerConfiguration<C, B>> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        EqualizerSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<crate::api::settings::Setting> {
        let equalizer_configuration = state.get();
        let setting = (*setting_id).try_into().ok()?;
        Some(match setting {
            EqualizerSetting::PresetEqualizerProfile => {
                Setting::optional_select_from_enum_all_variants(
                    equalizer_configuration.preset_profile(),
                )
            }
            EqualizerSetting::CustomEqualizerProfile => Setting::ModifiableSelect {
                setting: {
                    let custom_profiles = self.custom_profiles_receiver.borrow();
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
                        self.custom_profiles_receiver
                            .borrow()
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
                value: equalizer_configuration
                    .volume_adjustments_channel_1()
                    .adjustments()
                    .to_vec(),
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let equalizer_configuration = state.get_mut();
        let setting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            EqualizerSetting::PresetEqualizerProfile => {
                if let Some(preset) = value.try_as_optional_enum_variant()? {
                    *equalizer_configuration = EqualizerConfiguration::new_from_preset_profile(
                        preset,
                        equalizer_configuration
                            .volume_adjustments()
                            .map(|v| v.adjustments().iter().copied().skip(8).collect()),
                    );
                } else {
                    *equalizer_configuration = EqualizerConfiguration::new_custom_profile(
                        equalizer_configuration.volume_adjustments().to_owned(),
                    );
                }
            }
            EqualizerSetting::CustomEqualizerProfile => {
                if let Ok(name) = value.try_as_str() {
                    if let Some(volume_adjustments) = self
                        .custom_profiles_receiver
                        .borrow()
                        .iter()
                        .find(|(n, _)| n == name)
                        .map(|(_, volume_adjustments)| volume_adjustments)
                    {
                        *state.get_mut() = EqualizerConfiguration::new_custom_profile(
                            self.values_to_volume_adjustments(
                                volume_adjustments,
                                equalizer_configuration.volume_adjustments(),
                            ),
                        );
                    }
                } else if let Value::ModifiableSelectCommand(command) = value {
                    match command {
                        settings::ModifiableSelectCommand::Add(name) => {
                            self.profile_store
                                .upsert(
                                    name.into_owned(),
                                    equalizer_configuration
                                        .volume_adjustments_channel_1()
                                        .adjustments()
                                        .to_vec(),
                                )
                                .await?;
                        }
                        settings::ModifiableSelectCommand::Remove(name) => {
                            self.profile_store.delete(name.into_owned()).await?;
                        }
                    }
                }
            }
            EqualizerSetting::VolumeAdjustments => {
                let volume_adjustments = value.try_as_i16_slice()?;
                *equalizer_configuration =
                    EqualizerConfiguration::new_custom_profile(self.values_to_volume_adjustments(
                        volume_adjustments,
                        equalizer_configuration.volume_adjustments(),
                    ));
            }
        }
        Ok(())
    }
}
