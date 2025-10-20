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

pub struct ImportExportSettingHandler<const C: usize, const B: usize> {
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

impl<const C: usize, const B: usize> ImportExportSettingHandler<C, B> {
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
}

#[async_trait]
impl<T, const C: usize, const B: usize> SettingHandler<T> for ImportExportSettingHandler<C, B>
where
    T: Has<EqualizerConfiguration<C, B>> + Send,
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
                        volume_adjustments: values.iter().map(|i| *i as f64 / 10f64).collect(),
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
                let profiles = exported_profiles
                    .into_iter()
                    .map(|exported| {
                        (
                            exported.name.into_owned(),
                            exported
                                .volume_adjustments
                                .into_iter()
                                .map(|value| (value * 10f64).round() as i16)
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect();
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
