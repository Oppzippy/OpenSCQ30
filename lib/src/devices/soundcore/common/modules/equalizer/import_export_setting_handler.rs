use std::{
    borrow::Cow,
    collections::HashSet,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use openscq30_lib_has::Has;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use tokio::sync::watch;
use tracing::instrument;

use crate::{
    api::settings::{self, SettingId, Value},
    devices::soundcore::common::{
        modules::equalizer::custom_equalizer_profile_store::CustomEqualizerProfileStore,
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::EqualizerConfiguration,
    },
    i18n::fl,
};

use super::ImportExportSetting;

pub struct ImportExportSettingHandler<
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    profile_store: Arc<CustomEqualizerProfileStore>,
    profiles_receiver: watch::Receiver<Vec<(String, Vec<i16>)>>,
    selected_profiles: Mutex<HashSet<String>>,
    change_notify: watch::Sender<()>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportedCustomProfile<'a> {
    pub name: Cow<'a, str>,
    pub volume_adjustments: Vec<f64>,
}

impl<
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>
    ImportExportSettingHandler<
        CHANNELS,
        BANDS,
        VISIBLE_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
{
    #[instrument(skip(profile_store))]
    pub fn new(
        profile_store: Arc<CustomEqualizerProfileStore>,
        change_notify: watch::Sender<()>,
    ) -> Self {
        Self {
            profiles_receiver: profile_store.subscribe(),
            profile_store,
            change_notify,
            selected_profiles: Default::default(),
        }
    }

    /// Converts an imported profile to the format we store, making sure it has exactly the bands the device has and
    /// that every value is one the device accepts. Profiles that don't would otherwise be stored and then break
    /// everything that reads the stored profiles.
    fn parse_imported_profile(
        exported: ExportedCustomProfile,
    ) -> SettingHandlerResult<(String, Vec<i16>)> {
        let ExportedCustomProfile {
            name,
            volume_adjustments,
        } = exported;
        if volume_adjustments.len() != VISIBLE_BANDS {
            return Err(SettingHandlerError::Other(
                format!(
                    "profile \"{name}\" has {} bands, but this device's equalizer has {VISIBLE_BANDS}",
                    volume_adjustments.len(),
                )
                .into(),
            ));
        }
        let scale = 10u32.pow(FRACTION_DIGITS.into()) as f64;
        let allowed_range = f64::from(MIN_VOLUME)..=f64::from(MAX_VOLUME);
        let stored_volume_adjustments = volume_adjustments
            .iter()
            .enumerate()
            .map(|(band, value)| {
                let scaled = (value * scale).round();
                // written as a negated contains so that NaN is rejected too
                if !allowed_range.contains(&scaled) {
                    return Err(SettingHandlerError::Other(
                        format!(
                            "profile \"{name}\" has {value} for band {}, but values must be between {} and {}",
                            band + 1,
                            f64::from(MIN_VOLUME) / scale,
                            f64::from(MAX_VOLUME) / scale,
                        )
                        .into(),
                    ));
                }
                Ok(scaled as i16)
            })
            .collect::<SettingHandlerResult<Vec<_>>>()?;
        Ok((name.into_owned(), stored_volume_adjustments))
    }
}

#[async_trait]
impl<
    T,
    const CHANNELS: usize,
    const BANDS: usize,
    const VISIBLE_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> SettingHandler<T>
    for ImportExportSettingHandler<
        CHANNELS,
        BANDS,
        VISIBLE_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
where
    T: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ImportExportSetting::iter().map(Into::into).collect()
    }

    fn get(&self, _state: &T, setting_id: &SettingId) -> Option<settings::Setting> {
        let setting = (*setting_id).try_into().ok()?;
        Some(match setting {
            ImportExportSetting::ImportCustomEqualizerProfiles => settings::Setting::ImportString {
                confirmation_message: Some(fl!("import-custom-equalizer-profiles-confirm")),
            },
            ImportExportSetting::ExportCustomEqualizerProfiles => {
                let profile_names = self
                    .profiles_receiver
                    .borrow()
                    .iter()
                    .map(|(name, _)| name)
                    .cloned()
                    .collect::<Vec<_>>();
                let selected_profiles = self.selected_profiles.lock().unwrap();
                settings::Setting::MultiSelect {
                    values: profile_names
                        .iter()
                        .filter(|name| selected_profiles.contains(*name))
                        .cloned()
                        .map(Cow::from)
                        .collect(),
                    setting: settings::Select {
                        options: profile_names.iter().cloned().map(Cow::Owned).collect(),
                        localized_options: profile_names,
                    },
                }
            }
            ImportExportSetting::ExportCustomEqualizerProfilesOutput => {
                let custom_profiles = self.profiles_receiver.borrow();
                let selection = self.selected_profiles.lock().unwrap();
                let exported_profiles = custom_profiles
                    .iter()
                    .filter(|(name, _)| selection.contains(name))
                    .map(|(name, values)| ExportedCustomProfile {
                        name: name.into(),
                        volume_adjustments: values
                            .iter()
                            .map(|i| *i as f64 / 10u32.pow(FRACTION_DIGITS.into()) as f64)
                            .collect(),
                    })
                    .collect::<Vec<_>>();
                let json = serde_json::to_string(&exported_profiles).unwrap();
                settings::Setting::Information {
                    value: json.to_owned(),
                    translated_value: json,
                }
            }
        })
    }

    async fn set(
        &self,
        _state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let setting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            ImportExportSetting::ImportCustomEqualizerProfiles => {
                let json = value.try_as_str()?;
                let exported_profiles: Vec<ExportedCustomProfile> = serde_json::from_str(json)
                    .map_err(|err| SettingHandlerError::Other(Box::new(err)))?;
                // validate everything before storing anything, so a bad profile doesn't result in a partial import
                let profiles = exported_profiles
                    .into_iter()
                    .map(Self::parse_imported_profile)
                    .collect::<SettingHandlerResult<Vec<_>>>()?;
                self.profile_store.bulk_upsert(profiles).await?;
            }
            ImportExportSetting::ExportCustomEqualizerProfiles => {
                let values = value.try_into_string_vec()?;
                *self.selected_profiles.lock().unwrap() =
                    values.into_iter().map(|cow| cow.into_owned()).collect();
                self.change_notify.send_replace(());
            }
            ImportExportSetting::ExportCustomEqualizerProfilesOutput => {
                return Err(SettingHandlerError::ReadOnly);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        DeviceModel,
        devices::soundcore::common::structures::{
            CommonEqualizerConfiguration, EqualizerConfiguration, VolumeAdjustments,
        },
        storage::OpenSCQ30Database,
    };

    use super::*;

    #[derive(openscq30_lib_macros::Has)]
    struct TestStateWithEq {
        pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    }

    type TestHandler = ImportExportSettingHandler<2, 10, 8, -120, 134, 1>;

    async fn set_up() -> (TestHandler, Arc<CustomEqualizerProfileStore>) {
        let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());
        let (change_notify_sender, _) = watch::channel(());
        let profile_store = Arc::new(
            CustomEqualizerProfileStore::new(
                database,
                DeviceModel::SoundcoreDevelopment,
                change_notify_sender.clone(),
            )
            .await,
        );
        let handler = TestHandler::new(profile_store.clone(), change_notify_sender);
        (handler, profile_store)
    }

    async fn import(handler: &TestHandler, json: &str) -> Result<(), SettingHandlerError> {
        let mut state = TestStateWithEq {
            equalizer_configuration: EqualizerConfiguration::new_all_bands_present(
                0xfefe,
                [VolumeAdjustments::new([0; 10]); 2],
            ),
        };
        handler
            .set(
                &mut state,
                &SettingId::ImportCustomEqualizerProfiles,
                Value::String(json.to_owned().into()),
            )
            .await
    }

    fn stored_profiles(profile_store: &CustomEqualizerProfileStore) -> Vec<(String, Vec<i16>)> {
        profile_store.subscribe().borrow().clone()
    }

    #[tokio::test(start_paused = true)]
    async fn import_rejects_short_profile() {
        let (handler, profile_store) = set_up().await;
        let err = import(
            &handler,
            r#"[{"name":"Short profile","volumeAdjustments":[0,1,2]}]"#,
        )
        .await
        .expect_err("a profile with too few bands should be rejected");
        assert!(matches!(err, SettingHandlerError::Other(_)), "{err:?}");
        let message = err.to_string();
        assert!(message.contains("Short profile"), "{message}");
        assert!(stored_profiles(&profile_store).is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn import_rejects_profile_with_too_many_bands() {
        let (handler, profile_store) = set_up().await;
        let err = import(
            &handler,
            r#"[{"name":"Long profile","volumeAdjustments":[0,0,0,0,0,0,0,0,0]}]"#,
        )
        .await
        .expect_err("a profile with too many bands should be rejected");
        assert!(matches!(err, SettingHandlerError::Other(_)), "{err:?}");
        assert!(stored_profiles(&profile_store).is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn import_rejects_out_of_range_values() {
        for out_of_range in ["13.5", "-12.1", "1000000"] {
            let (handler, profile_store) = set_up().await;
            let json = format!(
                r#"[{{"name":"Loud profile","volumeAdjustments":[0,0,0,0,0,0,0,{out_of_range}]}}]"#
            );
            let err = import(&handler, &json)
                .await
                .expect_err("a profile with values outside the supported range should be rejected");
            assert!(matches!(err, SettingHandlerError::Other(_)), "{err:?}");
            let message = err.to_string();
            assert!(message.contains("Loud profile"), "{message}");
            assert!(stored_profiles(&profile_store).is_empty());
        }
    }

    #[tokio::test(start_paused = true)]
    async fn import_is_all_or_nothing() {
        let (handler, profile_store) = set_up().await;
        import(
            &handler,
            r#"[
                {"name":"Good profile","volumeAdjustments":[0,0,0,0,0,0,0,0]},
                {"name":"Short profile","volumeAdjustments":[0,1,2]}
            ]"#,
        )
        .await
        .expect_err("a single invalid profile should reject the whole import");
        assert!(stored_profiles(&profile_store).is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn import_accepts_profiles_at_the_range_limits() {
        let (handler, profile_store) = set_up().await;
        import(
            &handler,
            r#"[{"name":"Extreme profile","volumeAdjustments":[-12,13.4,-12,13.4,-12,13.4,-12,13.4]}]"#,
        )
        .await
        .unwrap();
        assert_eq!(
            stored_profiles(&profile_store),
            vec![(
                "Extreme profile".to_owned(),
                vec![-120, 134, -120, 134, -120, 134, -120, 134]
            )]
        );
    }
}
