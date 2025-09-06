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

    /// Returns all saved quick presets for the device.
    pub async fn quick_presets(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
    ) -> storage::Result<Vec<QuickPreset>> {
        self.database.fetch_all_quick_presets(device.model()).await
    }

    /// Saves the device's current settings to a quick preset. By default, all fields will be toggled off. If a quick
    /// preset with the specified name already exists, the setting values will be updated, but whether or not each
    /// field is toggled on or not will remain unchanged.
    pub async fn save(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
    ) -> storage::Result<()> {
        let fields = device
            .categories()
            .iter()
            .flat_map(|category_id| device.settings_in_category(category_id))
            // TODO make the SettingId blacklist a part of OpenSCQ30Device?
            .filter(|setting_id| {
                !matches!(
                    setting_id,
                    SettingId::ImportCustomEqualizerProfiles
                        | SettingId::ExportCustomEqualizerProfiles,
                )
            })
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

    /// Toggles on/off a field for the specified quick preset. When a quick preset is activated, only the enabled
    /// fields will be set.
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

    /// Activates a quick preset, setting all enabled fields' settings.
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

    /// Deletes a quick preset.
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
