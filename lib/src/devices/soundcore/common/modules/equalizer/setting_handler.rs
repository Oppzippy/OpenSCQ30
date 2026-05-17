use std::{array, borrow::Cow, sync::Arc};

use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;
use tokio::sync::watch;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::equalizer::{
            EqualizerPreset, custom_equalizer_profile_store::CustomEqualizerProfileStore,
        },
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::{EqualizerConfiguration, TwsStatus, VolumeAdjustments},
    },
};

use super::EqualizerSetting;

pub struct EqualizerSettingHandler<
    StateT,
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const PRESET_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    profile_store: Arc<CustomEqualizerProfileStore>,
    custom_profiles_receiver: watch::Receiver<Vec<(String, Vec<i16>)>>,
    get_tws_status: Option<fn(&StateT) -> TwsStatus>,
    custom_preset_id: u16,
    band_hz: [u16; VISIBLE_BANDS],
    presets: Vec<EqualizerPreset<PRESET_BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>,
}

impl<
    StateT,
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const PRESET_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>
    EqualizerSettingHandler<
        StateT,
        CHANNELS,
        BANDS,
        VISIBLE_BANDS,
        PRESET_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
{
    pub fn new(
        profile_store: Arc<CustomEqualizerProfileStore>,
        custom_preset_id: u16,
        band_hz: [u16; VISIBLE_BANDS],
        presets: Vec<EqualizerPreset<PRESET_BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>,
    ) -> Self {
        const {
            assert!(
                VISIBLE_BANDS <= BANDS,
                "there can't be more visible bands than there are total bands",
            );
            assert!(
                PRESET_BANDS <= BANDS,
                "there can't be more preset bands than there are total bands",
            );
            assert!(
                PRESET_BANDS >= VISIBLE_BANDS,
                "there can't be fewer preset bands than visible bands",
            );
        }
        Self {
            custom_profiles_receiver: profile_store.subscribe(),
            profile_store,
            get_tws_status: None,
            custom_preset_id,
            band_hz,
            presets,
        }
    }

    pub fn with_tws(mut self) -> Self
    where
        StateT: Has<TwsStatus>,
    {
        self.get_tws_status = Some(|state| *state.get());
        self
    }
}

#[async_trait]
impl<
    StateT,
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const PRESET_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> SettingHandler<StateT>
    for EqualizerSettingHandler<
        StateT,
        CHANNELS,
        BANDS,
        VISIBLE_BANDS,
        PRESET_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
where
    StateT: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
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

        get_inner(
            equalizer_configuration,
            self.band_hz,
            &self.presets,
            self.custom_preset_id,
            &self.custom_profiles_receiver,
            setting_id,
        )
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
        set_inner(
            equalizer_configuration,
            &self.presets,
            self.custom_preset_id,
            &self.custom_profiles_receiver,
            &self.profile_store,
            setting_id,
            value,
        )
        .await
    }
}

#[inline(never)]
fn get_inner<
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const PRESET_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    equalizer_configuration: &EqualizerConfiguration<
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
    band_hz: [u16; VISIBLE_BANDS],
    presets: &[EqualizerPreset<PRESET_BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>],
    custom_preset_id: u16,
    custom_profiles_receiver: &watch::Receiver<Vec<(String, Vec<i16>)>>,
    setting_id: &SettingId,
) -> Option<crate::api::settings::Setting> {
    let setting = (*setting_id).try_into().ok()?;
    Some(match setting {
        EqualizerSetting::PresetEqualizerProfile => {
            let maybe_preset = presets
                .iter()
                .find(|preset| preset.id == equalizer_configuration.preset_id())
                .copied();
            Setting::OptionalSelect {
                setting: settings::Select {
                    options: presets
                        .iter()
                        .map(|preset| Cow::Borrowed(preset.name))
                        .collect(),
                    localized_options: presets
                        .iter()
                        .map(|preset| (preset.localized_name)())
                        .collect(),
                },
                value: maybe_preset.map(|preset| Cow::Borrowed(preset.name)),
            }
        }
        EqualizerSetting::CustomEqualizerProfile => {
            let custom_profiles = custom_profiles_receiver.borrow();
            Setting::ModifiableSelect {
                setting: {
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
                value: (equalizer_configuration.preset_id() == custom_preset_id)
                    .then(|| {
                        custom_profiles
                            .iter()
                            .find(|(_, v)| {
                                v == equalizer_configuration
                                    .volume_adjustments_channel_1()
                                    .adjustments()
                            })
                            .map(|(name, _)| name.clone().into())
                    })
                    .flatten(),
            }
        }
        EqualizerSetting::VolumeAdjustments => Setting::Equalizer {
            setting: settings::Equalizer {
                band_hz: Cow::Owned(band_hz.to_vec()),
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

#[inline(never)]
async fn set_inner<
    const CHANNELS: usize,
    const BANDS: usize,
    const PRESET_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    equalizer_configuration: &mut EqualizerConfiguration<
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
    presets: &[EqualizerPreset<PRESET_BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>],
    custom_preset_id: u16,
    custom_profiles_receiver: &watch::Receiver<Vec<(String, Vec<i16>)>>,
    profile_store: &CustomEqualizerProfileStore,
    setting_id: &SettingId,
    value: Value,
) -> SettingHandlerResult<()> {
    let setting = (*setting_id)
        .try_into()
        .expect("already filtered to valid values only by SettingsManager");
    match setting {
        EqualizerSetting::PresetEqualizerProfile => {
            if let Some(preset) = value
                .try_as_optional_str()?
                .and_then(|preset_name| presets.iter().find(|it| it.name == preset_name))
            {
                *equalizer_configuration = EqualizerConfiguration::new(
                    preset.id,
                    equalizer_configuration.volume_adjustments().map(|v| {
                        VolumeAdjustments::new(array::from_fn(|i| {
                            preset
                                .volume_adjustments
                                .adjustments()
                                .get(i)
                                .copied()
                                .unwrap_or(v.adjustments()[i])
                        }))
                    }),
                );
            } else {
                *equalizer_configuration = EqualizerConfiguration::new(
                    custom_preset_id,
                    *equalizer_configuration.volume_adjustments(),
                );
            }
        }
        EqualizerSetting::CustomEqualizerProfile => {
            if let Ok(name) = value.try_as_str() {
                if let Some(volume_adjustments) = custom_profiles_receiver
                    .borrow()
                    .iter()
                    .find(|(n, _)| n == name)
                    .map(|(_, volume_adjustments)| volume_adjustments)
                {
                    *equalizer_configuration = EqualizerConfiguration::new(
                        custom_preset_id,
                        values_to_volume_adjustments(
                            volume_adjustments,
                            equalizer_configuration.volume_adjustments(),
                        ),
                    );
                }
            } else if let Value::ModifiableSelectCommand(command) = value {
                match command {
                    settings::ModifiableSelectCommand::Add(name) => {
                        profile_store
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
                        profile_store.delete(name.into_owned()).await?;
                    }
                }
            }
        }
        EqualizerSetting::VolumeAdjustments => {
            let volume_adjustments = value.try_as_i16_slice()?;
            *equalizer_configuration = EqualizerConfiguration::new(
                custom_preset_id,
                values_to_volume_adjustments(
                    volume_adjustments,
                    equalizer_configuration.volume_adjustments(),
                ),
            );
        }
    }
    Ok(())
}

fn values_to_volume_adjustments<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    values: &[i16],
    existing_volume_adjustments: &[VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>;
         CHANNELS],
) -> [VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>; CHANNELS] {
    // Some devices have extra bands, but those aren't exposed to the user, so I have no idea what they're for
    // We can just add back in whatever was there before (we're only showing the user the first 8 bands)
    array::from_fn(|band| {
        let band_adjustments = existing_volume_adjustments[band].adjustments();
        VolumeAdjustments::new(array::from_fn(|channel| {
            values
                .get(channel)
                .copied()
                .unwrap_or(band_adjustments[channel])
        }))
    })
}
