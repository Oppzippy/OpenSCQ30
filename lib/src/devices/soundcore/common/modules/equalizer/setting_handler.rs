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
        structures::{
            CustomEqualizerConfiguration, CustomVolumeAdjustments, PresetEqualizerProfile,
            TwsStatus,
        },
    },
};

use super::EqualizerSetting;

pub struct EqualizerSettingHandler<
    StateT,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    profile_store: Arc<CustomEqualizerProfileStore>,
    custom_profiles_receiver: watch::Receiver<Vec<(String, Vec<i16>)>>,
    get_tws_status: Option<fn(&StateT) -> TwsStatus>,
    custom_preset_id: u16,
    band_hz: &'static [u16],
}

impl<
    StateT,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> EqualizerSettingHandler<StateT, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
{
    #[instrument(skip(profile_store))]
    pub fn new(
        profile_store: Arc<CustomEqualizerProfileStore>,
        custom_preset_id: u16,
        band_hz: &'static [u16],
    ) -> Self {
        Self {
            custom_profiles_receiver: profile_store.subscribe(),
            profile_store,
            get_tws_status: None,
            custom_preset_id,
            band_hz,
        }
    }

    pub fn with_tws(mut self) -> Self
    where
        StateT: Has<TwsStatus>,
    {
        self.get_tws_status = Some(|state| *state.get());
        self
    }

    fn values_to_volume_adjustments(
        &self,
        values: &[i16],
        existing_volume_adjustments: &[CustomVolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>;
             CHANNELS],
    ) -> [CustomVolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>; CHANNELS] {
        // Some devices have extra bands, but those aren't exposed to the user, so I have no idea what they're for
        // We can just add back in whatever was there before (we're only showing the user the first 8 bands)
        array::from_fn(|band| {
            let band_adjustments = existing_volume_adjustments[band].adjustments();
            CustomVolumeAdjustments::new(array::from_fn(|channel| {
                values
                    .get(channel)
                    .copied()
                    .unwrap_or(band_adjustments[channel])
            }))
        })
    }
}

#[async_trait]
impl<
    StateT,
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> SettingHandler<StateT>
    for EqualizerSettingHandler<StateT, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
where
    StateT: Has<CustomEqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
        + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        EqualizerSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &StateT, setting_id: &SettingId) -> Option<crate::api::settings::Setting> {
        // TODO display as a read only setting. When TWS is disconnected, the equalizer configuration can be read
        // but not written.
        if let Some(get_tws_status) = self.get_tws_status
            && !get_tws_status(state).is_connected
        {
            return None;
        }

        let equalizer_configuration = state.get();
        let setting = (*setting_id).try_into().ok()?;
        Some(match setting {
            EqualizerSetting::PresetEqualizerProfile => {
                let preset = PresetEqualizerProfile::from_id(equalizer_configuration.preset_id());
                Setting::optional_select_from_enum_all_variants(preset)
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
                value: (equalizer_configuration.preset_id() == self.custom_preset_id)
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
                    band_hz: Cow::Borrowed(self.band_hz),
                    fraction_digits: FRACTION_DIGITS.into(),
                    min: MIN_VOLUME,
                    max: MAX_VOLUME,
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
        state: &mut StateT,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        // We can't modify the equalizer configuration while TWS is disconnected
        if let Some(get_tws_status) = self.get_tws_status
            && !get_tws_status(state).is_connected
        {
            return Ok(());
        }

        let equalizer_configuration = state.get_mut();
        let setting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            EqualizerSetting::PresetEqualizerProfile => {
                if let Some(preset) =
                    value.try_as_optional_enum_variant::<PresetEqualizerProfile>()?
                {
                    let preset_volume_adjustments = *preset.volume_adjustments().adjustments();
                    *equalizer_configuration = CustomEqualizerConfiguration::new(
                        preset.id(),
                        equalizer_configuration.volume_adjustments().map(|v| {
                            CustomVolumeAdjustments::new(array::from_fn(|i| {
                                preset_volume_adjustments
                                    .get(i)
                                    .copied()
                                    .unwrap_or(v.adjustments()[i])
                            }))
                        }),
                    );
                } else {
                    *equalizer_configuration = CustomEqualizerConfiguration::new(
                        self.custom_preset_id,
                        *equalizer_configuration.volume_adjustments(),
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
                        *state.get_mut() = CustomEqualizerConfiguration::new(
                            self.custom_preset_id,
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
                *equalizer_configuration = CustomEqualizerConfiguration::new(
                    self.custom_preset_id,
                    self.values_to_volume_adjustments(
                        volume_adjustments,
                        equalizer_configuration.volume_adjustments(),
                    ),
                );
            }
        }
        Ok(())
    }
}
