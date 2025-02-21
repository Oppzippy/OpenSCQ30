use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::storage::OpenSCQ30Database;

use super::{
    device::OpenSCQ30Device,
    settings::{SettingId, Value},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickPreset {
    pub name: String,
    pub is_active: bool,
    pub settings: Vec<QuickPresetField>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickPresetField {
    pub setting_id: SettingId<'static>,
    pub value: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuickPresetAction {
    Create(String, HashSet<SettingId<'static>>),
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
    ) -> crate::Result<Vec<QuickPreset>> {
        let all_setting_ids = device
            .categories()
            .iter()
            .flat_map(|category_id| device.settings_in_category(category_id))
            .collect::<Vec<_>>();

        Ok(self
            .database
            .fetch_all_quick_presets(device.model())
            .await?
            .into_iter()
            .map(|(name, settings)| QuickPreset {
                name,
                is_active: false,
                settings: all_setting_ids
                    .iter()
                    .map(|setting_id| QuickPresetField {
                        setting_id: setting_id.to_owned(),
                        value: settings.get(setting_id).cloned(),
                    })
                    .collect(),
            })
            .collect())
    }

    pub async fn save(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
        settings: HashMap<SettingId<'static>, Value>,
    ) -> crate::Result<()> {
        self.database
            .upsert_quick_preset(device.model(), name, settings)
            .await
            .map_err(Into::into)
    }

    pub async fn activate(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: impl Into<String>,
    ) -> crate::Result<()> {
        let settings = self
            .database
            .fetch_quick_preset(device.model(), name.into())
            .await?;
        device
            .set_setting_values(settings.into_iter().collect())
            .await
    }

    pub async fn delete(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
    ) -> crate::Result<()> {
        self.database
            .delete_quick_preset(device.model(), name)
            .await
            .map_err(Into::into)
    }
}
