use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::storage::{self, OpenSCQ30Database, QuickPreset, QuickPresetField};

use super::{
    device::{self, OpenSCQ30Device},
    settings::{self, SettingId},
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
    /// field is toggled on or not will remain unchanged. Fields for settings that are currently unavailable on the
    /// device are kept as they were.
    pub async fn save(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
    ) -> storage::Result<()> {
        let mut fields = device
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
                    // Actions are commands rather than state, so there is no value to save and restore.
                    .filter(|setting| setting.mode().is_writable() && !setting.is_action())
                    .map(|setting| QuickPresetField {
                        setting_id,
                        value: setting.into(),
                        is_enabled: false,
                    })
            })
            .collect::<Vec<_>>();

        // Some settings are only available in certain states, such as the noise canceling strength while in
        // transparency mode. Don't discard the saved value of a field just because it can't be read right now.
        let unavailable_fields = match self
            .database
            .fetch_quick_preset(device.model(), name.clone())
            .await
        {
            Ok(existing) => existing
                .fields
                .into_iter()
                .filter(|field| device.setting(&field.setting_id).is_none())
                .filter(|field| fields.iter().all(|f| f.setting_id != field.setting_id))
                .collect::<Vec<_>>(),
            Err(storage::Error::NotFound { .. }) => Vec::new(),
            Err(err) => return Err(err),
        };
        fields.extend(unavailable_fields);

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
        device
            .set_setting_values(settings_to_apply(device, quick_preset.fields))
            .await
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

/// Picks the enabled fields that should be set when activating a quick preset, in the order they should be set.
fn settings_to_apply(
    device: &(dyn OpenSCQ30Device + Send + Sync),
    fields: Vec<QuickPresetField>,
) -> Vec<(SettingId, settings::Value)> {
    fields
        .into_iter()
        .filter(|field| field.is_enabled)
        // Presets saved by older versions may contain actions, which would run every time the preset is activated.
        .filter(|field| {
            !device
                .setting(&field.setting_id)
                .is_some_and(|setting| setting.is_action())
        })
        .map(|field| (field.setting_id, field.value))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::DeviceModel;

    use super::*;

    async fn connect_demo_a3954() -> (
        Arc<dyn OpenSCQ30Device + Send + Sync>,
        Arc<OpenSCQ30Database>,
        QuickPresetsHandler,
    ) {
        let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());
        let model = DeviceModel::SoundcoreA3954;
        let registry = model.demo_device_registry(database.clone()).await.unwrap();
        let device = registry.connect(model.demo_mac_address()).await.unwrap();
        let handler = QuickPresetsHandler::new(database.clone());
        (device, database, handler)
    }

    fn current_value(device: &dyn OpenSCQ30Device, setting_id: SettingId) -> settings::Value {
        device.setting(&setting_id).unwrap().into()
    }

    fn field(setting_id: SettingId, value: settings::Value, is_enabled: bool) -> QuickPresetField {
        QuickPresetField {
            setting_id,
            value,
            is_enabled,
        }
    }

    async fn fetch_field(
        database: &OpenSCQ30Database,
        name: &str,
        setting_id: SettingId,
    ) -> Option<QuickPresetField> {
        database
            .fetch_quick_preset(DeviceModel::SoundcoreA3954, name.into())
            .await
            .unwrap()
            .fields
            .into_iter()
            .find(|field| field.setting_id == setting_id)
    }

    #[tokio::test(start_paused = true)]
    async fn save_does_not_capture_action_settings() {
        let (device, database, handler) = connect_demo_a3954().await;

        handler
            .save(device.as_ref(), "Preset".into())
            .await
            .unwrap();

        assert_eq!(
            None,
            fetch_field(&database, "Preset", SettingId::ResetButtonsToDefault).await,
            "actions have no state, so they shouldn't be saved",
        );
        assert!(
            fetch_field(&database, "Preset", SettingId::LeftSinglePress)
                .await
                .is_some(),
            "regular settings should still be saved",
        );
    }

    #[tokio::test(start_paused = true)]
    async fn activate_does_not_run_action_settings() {
        let (device, database, handler) = connect_demo_a3954().await;
        let original = current_value(device.as_ref(), SettingId::LeftSinglePress);
        device
            .set_setting_values(vec![(SettingId::LeftSinglePress, Some("NextSong").into())])
            .await
            .unwrap();
        assert_ne!(
            original,
            current_value(device.as_ref(), SettingId::LeftSinglePress),
            "the button should have been changed for the test to mean anything",
        );
        // Older versions saved actions as if they were toggles, so presets like this can already exist.
        database
            .upsert_quick_preset(
                DeviceModel::SoundcoreA3954,
                QuickPreset {
                    name: "Preset".into(),
                    fields: vec![field(SettingId::ResetButtonsToDefault, false.into(), true)],
                },
            )
            .await
            .unwrap();

        handler.activate(device.as_ref(), "Preset").await.unwrap();

        assert_eq!(
            settings::Value::from(Some("NextSong")),
            current_value(device.as_ref(), SettingId::LeftSinglePress),
            "activating a quick preset shouldn't reset the buttons",
        );
    }

    #[tokio::test(start_paused = true)]
    async fn save_keeps_enabled_field_for_unavailable_setting() {
        let (device, database, handler) = connect_demo_a3954().await;
        device
            .set_setting_values(vec![
                (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
                (SettingId::ManualNoiseCanceling, 5.into()),
            ])
            .await
            .unwrap();
        handler.save(device.as_ref(), "ANC".into()).await.unwrap();
        handler
            .toggle_field(
                device.as_ref(),
                "ANC".into(),
                SettingId::ManualNoiseCanceling,
                true,
            )
            .await
            .unwrap();

        // The noise canceling strength can't be read in transparency mode
        device
            .set_setting_values(vec![(SettingId::AmbientSoundMode, "Transparency".into())])
            .await
            .unwrap();
        assert!(device.setting(&SettingId::ManualNoiseCanceling).is_none());
        handler.save(device.as_ref(), "ANC".into()).await.unwrap();

        assert_eq!(
            Some(field(SettingId::ManualNoiseCanceling, 5.into(), true)),
            fetch_field(&database, "ANC", SettingId::ManualNoiseCanceling).await,
            "the field should be unchanged since there was no new value to save",
        );
        assert_eq!(
            Some(settings::Value::from("Transparency")),
            fetch_field(&database, "ANC", SettingId::AmbientSoundMode)
                .await
                .map(|field| field.value),
            "available fields should still be updated",
        );
    }

    #[tokio::test(start_paused = true)]
    async fn save_new_preset_does_not_invent_unavailable_fields() {
        let (device, database, handler) = connect_demo_a3954().await;
        device
            .set_setting_values(vec![(SettingId::AmbientSoundMode, "Transparency".into())])
            .await
            .unwrap();

        handler.save(device.as_ref(), "New".into()).await.unwrap();

        assert_eq!(
            None,
            fetch_field(&database, "New", SettingId::ManualNoiseCanceling).await,
        );
    }
}
