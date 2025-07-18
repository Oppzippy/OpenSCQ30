use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::storage::{self, OpenSCQ30Database, QuickPreset, QuickPresetField};

use super::{
    device::{self, OpenSCQ30Device},
    settings::SettingId,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuickPresetAction {
    Create(String, HashSet<SettingId>),
    Activate(String),
    Delete(String),
}

#[derive(Debug, Clone)]
pub struct QuickPresetsHandler {
    database: Arc<OpenSCQ30Database>,
}

impl QuickPresetsHandler {
    pub fn new(database: Arc<OpenSCQ30Database>) -> Self {
        Self { database }
    }

    pub async fn quick_presets(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
    ) -> storage::Result<Vec<QuickPreset>> {
        self.database.fetch_all_quick_presets(device.model()).await
    }

    pub async fn save(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
    ) -> storage::Result<()> {
        let fields = device
            .categories()
            .iter()
            .flat_map(|category_id| device.settings_in_category(category_id))
            .filter_map(|setting_id| {
                device
                    .setting(&setting_id)
                    .filter(|setting| setting.mode().is_writable())
                    .map(|setting| QuickPresetField {
                        setting_id,
                        value: setting.into(),
                        is_enabled: false,
                    })
            })
            .collect::<Vec<_>>();

        self.database
            .upsert_quick_preset(device.model(), QuickPreset { name, fields })
            .await
    }

    pub async fn toggle_field(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
        setting_id: SettingId,
        is_enabled: bool,
    ) -> storage::Result<()> {
        self.database
            .toggle_quick_preset_field(device.model(), name, setting_id, is_enabled)
            .await
    }

    pub async fn activate(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: impl Into<String>,
    ) -> device::Result<()> {
        let quick_preset = self
            .database
            .fetch_quick_preset(device.model(), name.into())
            .await?;
        let settings = quick_preset
            .fields
            .into_iter()
            .filter_map(|field| field.is_enabled.then_some((field.setting_id, field.value)))
            .collect::<Vec<_>>();
        device.set_setting_values(settings).await
    }

    pub async fn delete(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
    ) -> storage::Result<()> {
        self.database
            .delete_quick_preset(device.model(), name)
            .await
    }
}
