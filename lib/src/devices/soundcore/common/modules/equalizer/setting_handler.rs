use std::{array, borrow::Cow, sync::Arc};

use async_trait::async_trait;
use itertools::Itertools;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;
use tokio::sync::watch;

use crate::{
    api::settings::{self, Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::equalizer::{
            EqualizerModuleSettings, InvisibleBandsMode,
            custom_equalizer_profile_store::CustomEqualizerProfileStore,
        },
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
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
    module_settings: EqualizerModuleSettings<
        VISIBLE_BANDS,
        PRESET_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
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
        module_settings: EqualizerModuleSettings<
            VISIBLE_BANDS,
            PRESET_BANDS,
            MIN_VOLUME,
            MAX_VOLUME,
            FRACTION_DIGITS,
        >,
    ) -> Self {
        const {
            assert!(
                VISIBLE_BANDS <= BANDS,
                "there can't be more visible bands than there are total bands",
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
            module_settings,
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
            &self.module_settings,
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
            &self.module_settings,
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
    module_settings: &EqualizerModuleSettings<
        VISIBLE_BANDS,
        PRESET_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
    custom_profiles_receiver: &watch::Receiver<Vec<(String, Vec<i16>)>>,
    setting_id: &SettingId,
) -> Option<crate::api::settings::Setting> {
    let setting = (*setting_id).try_into().ok()?;
    Some(match setting {
        EqualizerSetting::PresetEqualizerProfile => {
            let maybe_preset = module_settings
                .presets
                .iter()
                .find(|preset| preset.id == equalizer_configuration.preset_id())
                .copied();
            Setting::OptionalSelect {
                setting: settings::Select {
                    options: module_settings
                        .presets
                        .iter()
                        .map(|preset| Cow::Borrowed(preset.name))
                        .collect(),
                    localized_options: module_settings
                        .presets
                        .iter()
                        .map(|preset| (preset.localized_name)())
                        .collect(),
                },
                value: maybe_preset.map(|preset| Cow::Borrowed(preset.name)),
            }
        }
        EqualizerSetting::CustomEqualizerProfile => {
            let Some(custom_preset_id) = module_settings.custom_preset_id else {
                return None;
            };
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
                                v[..VISIBLE_BANDS]
                                    == equalizer_configuration
                                        .volume_adjustments_channel_1()
                                        .copied()
                                        .unwrap_or_default()
                                        .adjustments()[..VISIBLE_BANDS]
                            })
                            .map(|(name, _)| name.clone().into())
                    })
                    .flatten(),
            }
        }
        EqualizerSetting::VolumeAdjustments => Setting::Equalizer {
            setting: settings::Equalizer {
                band_hz: Cow::Owned(module_settings.band_hz.to_vec()),
                fraction_digits: FRACTION_DIGITS.into(),
                min: MIN_VOLUME,
                max: MAX_VOLUME,
            },
            read_only: module_settings.custom_preset_id.is_none(),
            value: equalizer_configuration
                .volume_adjustments_channel_1()
                .copied()
                .unwrap_or_default()
                .adjustments()[..VISIBLE_BANDS]
                .to_vec(),
        },
    })
}

#[inline(never)]
async fn set_inner<
    const VISIBLE_BANDS: usize,
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
    module_settings: &EqualizerModuleSettings<
        VISIBLE_BANDS,
        PRESET_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
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
            if let Some(preset) = value.try_as_optional_str()?.and_then(|preset_name| {
                module_settings
                    .presets
                    .iter()
                    .find(|it| it.name == preset_name)
            }) {
                *equalizer_configuration = EqualizerConfiguration::new_all_bands_present(
                    preset.id,
                    values_to_volume_adjustments(
                        // preset values should override invisible band handling
                        // this is done by passing an array of PRESET_BANDS length, so VISIBLE_BANDS will be
                        // PRESET_BANDS
                        preset.volume_adjustments.adjustments(),
                        &module_settings.invisible_bands_mode,
                    ),
                );
            }
        }
        EqualizerSetting::CustomEqualizerProfile => {
            let Some(custom_preset_id) = module_settings.custom_preset_id else {
                // TODO return error
                return Ok(());
            };
            if let Ok(name) = value.try_as_str() {
                if let Some(volume_adjustments) = custom_profiles_receiver
                    .borrow()
                    .iter()
                    .find(|(n, _)| n == name)
                    .map(|(_, volume_adjustments)| volume_adjustments)
                {
                    // it's possible a different OpenSCQ30 version saved a different number of bands than
                    // we expect, so make sure we have the exact number we want
                    let fixed_len_volume_adjustments: [i16; VISIBLE_BANDS] = volume_adjustments
                        .iter()
                        .copied()
                        .take(VISIBLE_BANDS)
                        .chain(std::iter::repeat_n(
                            0,
                            VISIBLE_BANDS.saturating_sub(volume_adjustments.len()),
                        ))
                        .collect_array()
                        .expect("we made sure there are exactly VISIBLE_BANDS elements");
                    *equalizer_configuration = EqualizerConfiguration::new_all_bands_present(
                        custom_preset_id,
                        values_to_volume_adjustments(
                            &fixed_len_volume_adjustments,
                            &module_settings.invisible_bands_mode,
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
                                    .copied()
                                    .unwrap_or_default()
                                    .adjustments()
                                    .iter()
                                    .copied()
                                    .take(VISIBLE_BANDS)
                                    .collect(),
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
            let Some(custom_preset_id) = module_settings.custom_preset_id else {
                // We want to display the equalizer but not accept any changes if custom profiles aren't supported,
                // so don't error, just don't make any changes.
                return Err(SettingHandlerError::ReadOnly);
            };
            let volume_adjustments = value.try_as_i16_array::<VISIBLE_BANDS>()?;
            *equalizer_configuration = EqualizerConfiguration::new_all_bands_present(
                custom_preset_id,
                values_to_volume_adjustments(
                    volume_adjustments,
                    &module_settings.invisible_bands_mode,
                ),
            );
        }
    }
    Ok(())
}

fn values_to_volume_adjustments<
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    values: &[i16; VISIBLE_BANDS],
    mode: &InvisibleBandsMode,
) -> [VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>; CHANNELS] {
    // Some devices have extra bands, but those aren't exposed to the user, so I have no idea what they're for
    [match mode {
        InvisibleBandsMode::Fixed(fixed) => VolumeAdjustments::new(array::from_fn(|band| {
            if let Some(value) = values.get(band) {
                *value
            } else {
                fixed.get(band - VISIBLE_BANDS).copied().unwrap_or_else(|| {
                    tracing::warn!(
                        "using fixed mode for invisible bands, but no value specified for band {band}"
                    );
                    0
                })
            }
        })),
    }; CHANNELS]
}

#[cfg(test)]
mod tests {
    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            modules::equalizer::common_settings, structures::CommonEqualizerConfiguration,
        },
        storage::OpenSCQ30Database,
    };

    use super::*;

    #[derive(openscq30_lib_macros::Has)]
    struct TestStateWithEq {
        pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    }

    async fn set_up() -> (
        Arc<OpenSCQ30Database>,
        EqualizerSettingHandler<TestStateWithEq, 2, 10, 8, 10, -120, 134, 1>,
        Arc<CustomEqualizerProfileStore>,
    ) {
        let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());
        let (change_notify_sender, _) = watch::channel(());
        let profile_store = Arc::new(
            CustomEqualizerProfileStore::new(
                database.clone(),
                DeviceModel::SoundcoreDevelopment,
                change_notify_sender,
            )
            .await,
        );
        let setting_handler =
            EqualizerSettingHandler::<TestStateWithEq, 2, 10, 8, 10, -120, 134, 1>::new(
                profile_store.clone(),
                common_settings(),
            );
        (database, setting_handler, profile_store)
    }

    #[tokio::test(start_paused = true)]
    async fn custom_profiles_only_save_visible_bands() {
        let (database, setting_handler, _) = set_up().await;
        let mut state = TestStateWithEq {
            equalizer_configuration: EqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::new([0; 10]); 2],
            ),
        };
        setting_handler
            .set(
                &mut state,
                &SettingId::CustomEqualizerProfile,
                Value::ModifiableSelectCommand(settings::ModifiableSelectCommand::Add(
                    "test profile".into(),
                )),
            )
            .await
            .unwrap();
        let custom_profile = database
            .fetch_equalizer_profile(DeviceModel::SoundcoreDevelopment, "test profile".to_owned())
            .await
            .expect("we just created the custom profile, so it should exist");
        assert_eq!(custom_profile, [0; 8])
    }

    #[tokio::test(start_paused = true)]
    async fn activating_custom_profiles_with_more_bands_than_visible_ignores_invisible_bands() {
        let (_database, setting_handler, profile_store) = set_up().await;
        profile_store
            .bulk_upsert(vec![("test profile".to_owned(), vec![1; 10])])
            .await
            .unwrap();
        let mut state = TestStateWithEq {
            equalizer_configuration: EqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::new([0; 10]); 2],
            ),
        };
        setting_handler
            .set(
                &mut state,
                &SettingId::CustomEqualizerProfile,
                Value::String("test profile".into()),
            )
            .await
            .unwrap();
        assert_eq!(
            state.equalizer_configuration,
            EqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::new([1, 1, 1, 1, 1, 1, 1, 1, 0, 0]); 2]
            ),
        )
    }

    #[tokio::test(start_paused = true)]
    async fn activating_custom_profiles_with_too_few_bands_infers_0() {
        let (_database, setting_handler, profile_store) = set_up().await;
        profile_store
            .bulk_upsert(vec![("test profile".to_owned(), vec![1; 1])])
            .await
            .unwrap();
        let mut state = TestStateWithEq {
            equalizer_configuration: EqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::new([2; 10]); 2],
            ),
        };
        setting_handler
            .set(
                &mut state,
                &SettingId::CustomEqualizerProfile,
                Value::String("test profile".into()),
            )
            .await
            .unwrap();
        assert_eq!(
            state.equalizer_configuration,
            EqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::new([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]); 2]
            ),
        )
    }

    #[tokio::test(start_paused = true)]
    async fn newly_created_custom_profile_is_immediately_active() {
        let (_database, setting_handler, _) = set_up().await;
        let mut state = TestStateWithEq {
            equalizer_configuration: EqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::new([2; 10]); 2],
            ),
        };
        setting_handler
            .set(
                &mut state,
                &SettingId::VolumeAdjustments,
                Value::I16Vec(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            )
            .await
            .unwrap();
        setting_handler
            .set(
                &mut state,
                &SettingId::CustomEqualizerProfile,
                Value::ModifiableSelectCommand(settings::ModifiableSelectCommand::Add(
                    "test profile".into(),
                )),
            )
            .await
            .unwrap();
        let custom_profile_setting = setting_handler
            .get(&state, &SettingId::CustomEqualizerProfile)
            .unwrap();
        let Setting::ModifiableSelect {
            value: custom_profile,
            ..
        } = custom_profile_setting
        else {
            panic!("setting should be ModifiableSelect: {custom_profile_setting:?}");
        };

        assert_eq!(custom_profile, Some(Cow::from("test profile")));
    }
}
