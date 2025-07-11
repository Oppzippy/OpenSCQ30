use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use tokio::sync::watch;
use tracing::{instrument, warn};

use crate::{
    api::{
        device,
        settings::{self, SettingId, Value},
    },
    devices::{
        DeviceModel,
        soundcore::standard::{
            settings_manager::SettingHandler, structures::EqualizerConfiguration,
        },
    },
    storage::OpenSCQ30Database,
};

use super::ImportExportSetting;

pub struct ImportExportSettingHandler<const C: usize, const B: usize> {
    custom_profiles: Arc<Mutex<Vec<(String, Vec<i16>)>>>,
    selected_profiles: Mutex<Vec<String>>,
    change_notify: watch::Sender<()>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportedCustomProfile<'a> {
    pub name: Cow<'a, str>,
    pub volume_adjustments: Vec<f64>,
}

impl<const C: usize, const B: usize> ImportExportSettingHandler<C, B> {
    #[instrument(skip(database))]
    pub async fn new(
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) -> Self {
        let custom_profiles = database
            .fetch_all_equalizer_profiles(device_model)
            .await
            .unwrap_or_else(|err| {
                warn!("error fetching custom equalizer profiles, continuing without them: {err:?}");
                Vec::new()
            });

        let custom_profiles = Arc::new(Mutex::new(custom_profiles));
        // hack to work around bad design. we should instead somehow share state between setting handlers.
        {
            let mut notify = change_notify.subscribe();
            let custom_profiles = custom_profiles.to_owned();
            let database = database.to_owned();
            tokio::spawn(async move {
                while notify.changed().await.is_ok() {
                    *custom_profiles.lock().unwrap() = database
                        .fetch_all_equalizer_profiles(device_model)
                        .await
                        .unwrap();
                }
            });
        }
        Self {
            custom_profiles,
            change_notify,
            selected_profiles: Default::default(),
        }
    }
}

#[async_trait]
impl<T, const C: usize, const B: usize> SettingHandler<T> for ImportExportSettingHandler<C, B>
where
    T: AsMut<EqualizerConfiguration<C, B>> + AsRef<EqualizerConfiguration<C, B>> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ImportExportSetting::iter().map(Into::into).collect()
    }

    fn get(&self, _state: &T, setting_id: &SettingId) -> Option<settings::Setting> {
        let setting = setting_id.try_into().ok()?;
        Some(match setting {
            ImportExportSetting::ExportCustomProfiles => {
                let profile_names = self
                    .custom_profiles
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(name, _)| name)
                    .cloned()
                    .collect::<Vec<_>>();
                settings::Setting::MultiSelect {
                    setting: settings::Select {
                        options: profile_names.iter().cloned().map(Cow::Owned).collect(),
                        localized_options: profile_names,
                    },
                    values: self
                        .selected_profiles
                        .lock()
                        .unwrap()
                        .iter()
                        .cloned()
                        .map(Cow::Owned)
                        .collect(),
                }
            }
            ImportExportSetting::ExportCustomProfilesOutput => {
                let selection = self.selected_profiles.lock().unwrap();
                let custom_profiles = self.custom_profiles.lock().unwrap();
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
    ) -> device::Result<()> {
        let setting = setting_id
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            ImportExportSetting::ExportCustomProfiles => {
                let values = value.try_into_string_vec()?;
                *self.selected_profiles.lock().unwrap() =
                    values.into_iter().map(|cow| cow.into_owned()).collect();
                self.change_notify.send_replace(());
            }
            ImportExportSetting::ExportCustomProfilesOutput => {
                unimplemented!()
            }
        }
        Ok(())
    }
}
