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
    /// fields will be set. The equalizer can only be restored as either a factory profile or a custom curve, so
    /// enabling one of the fields that describe it disables the others.
    pub async fn toggle_field(
        &self,
        device: &(dyn OpenSCQ30Device + Send + Sync),
        name: String,
        setting_id: SettingId,
        is_enabled: bool,
    ) -> storage::Result<()> {
        if is_enabled && EQUALIZER_SETTING_IDS.contains(&setting_id) {
            for conflicting in EQUALIZER_SETTING_IDS
                .into_iter()
                .filter(|other| *other != setting_id)
            {
                match self
                    .database
                    .toggle_quick_preset_field(device.model(), name.clone(), conflicting, false)
                    .await
                {
                    // The preset doesn't have to contain every equalizer field.
                    Ok(()) | Err(storage::Error::NotFound { .. }) => (),
                    Err(err) => return Err(err),
                }
            }
        }
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
        let settings = settings_to_apply(device, quick_preset.fields);
        device.set_setting_values(settings.clone()).await?;

        // Setting one value can reset another one, such as changing the equalizer turning off spatial audio on some
        // devices. Set whatever didn't end up with the requested value once more, now that the other settings are in
        // place.
        let not_applied = settings
            .into_iter()
            .filter(|(setting_id, value)| {
                device
                    .setting(setting_id)
                    .is_some_and(|setting| settings::Value::from(setting) != *value)
            })
            .collect::<Vec<_>>();
        if not_applied.is_empty() {
            Ok(())
        } else {
            device.set_setting_values(not_applied).await
        }
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

/// The settings that all describe the same thing, the equalizer's curve, and so can't be restored independently.
const EQUALIZER_SETTING_IDS: [SettingId; 3] = [
    SettingId::PresetEqualizerProfile,
    SettingId::CustomEqualizerProfile,
    SettingId::VolumeAdjustments,
];

/// Picks the enabled fields that should be set when activating a quick preset, in the order they should be set.
fn settings_to_apply(
    device: &(dyn OpenSCQ30Device + Send + Sync),
    fields: Vec<QuickPresetField>,
) -> Vec<(SettingId, settings::Value)> {
    let mut settings = fields
        .into_iter()
        .filter(|field| field.is_enabled)
        // Presets saved by older versions may contain actions, which would run every time the preset is activated.
        .filter(|field| {
            !device
                .setting(&field.setting_id)
                .is_some_and(|setting| setting.is_action())
        })
        .map(|field| (field.setting_id, field.value))
        .collect::<Vec<_>>();

    // Setting the volume adjustments always switches to the custom profile, so if a factory profile and a curve are
    // both enabled, applying both would undo the factory profile. Presets saved by older versions may do this.
    let has_factory_profile = settings.iter().any(|(setting_id, value)| {
        *setting_id == SettingId::PresetEqualizerProfile
            && value.try_as_optional_str().is_ok_and(|name| name.is_some())
    });
    if has_factory_profile {
        settings.retain(|(setting_id, _)| {
            !matches!(
                setting_id,
                SettingId::CustomEqualizerProfile | SettingId::VolumeAdjustments
            )
        });
    }
    settings
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

    #[tokio::test(start_paused = true)]
    async fn eq_and_spatial_audio_are_applied_in_one_activation() {
        activate_eq_and_spatial_audio_preset(false).await;
    }

    #[tokio::test(start_paused = true)]
    async fn eq_and_spatial_audio_are_applied_in_one_activation_when_spatial_audio_is_on() {
        activate_eq_and_spatial_audio_preset(true).await;
    }

    /// Changing the equalizer turns off spatial audio on the A3954, which must not stop a quick preset from turning
    /// it on.
    async fn activate_eq_and_spatial_audio_preset(is_spatial_audio_on_initially: bool) {
        let (device, database, handler) = connect_demo_a3954().await;
        // SpatialAudio is registered by two modules on this device, so saving produces two fields with that setting
        // id and toggling either fails. Create the quick preset directly instead.
        database
            .upsert_quick_preset(
                DeviceModel::SoundcoreA3954,
                QuickPreset {
                    name: "Spatial".into(),
                    fields: vec![
                        field(SettingId::VolumeAdjustments, vec![10i16; 8].into(), true),
                        field(SettingId::SpatialAudio, true.into(), true),
                    ],
                },
            )
            .await
            .unwrap();
        device
            .set_setting_values(vec![(SettingId::VolumeAdjustments, vec![20i16; 8].into())])
            .await
            .unwrap();
        device
            .set_setting_values(vec![(
                SettingId::SpatialAudio,
                is_spatial_audio_on_initially.into(),
            )])
            .await
            .unwrap();
        assert_eq!(
            settings::Value::from(is_spatial_audio_on_initially),
            current_value(device.as_ref(), SettingId::SpatialAudio),
        );

        handler.activate(device.as_ref(), "Spatial").await.unwrap();

        assert_eq!(
            settings::Value::from(vec![10i16; 8]),
            current_value(device.as_ref(), SettingId::VolumeAdjustments),
        );
        assert_eq!(
            settings::Value::from(true),
            current_value(device.as_ref(), SettingId::SpatialAudio),
            "spatial audio should be on after a single activation",
        );
    }

    #[tokio::test(start_paused = true)]
    async fn factory_profile_wins_over_volume_adjustments() {
        let (device, database, handler) = connect_demo_a3954().await;
        device
            .set_setting_values(vec![(SettingId::VolumeAdjustments, vec![5i16; 8].into())])
            .await
            .unwrap();
        // Older versions allowed both to be enabled at the same time, even though setting the volume adjustments
        // always replaces the factory profile with a custom one.
        database
            .upsert_quick_preset(
                DeviceModel::SoundcoreA3954,
                QuickPreset {
                    name: "Acoustic".into(),
                    fields: vec![
                        field(
                            SettingId::PresetEqualizerProfile,
                            Some("Acoustic").into(),
                            true,
                        ),
                        field(SettingId::VolumeAdjustments, vec![1i16; 8].into(), true),
                    ],
                },
            )
            .await
            .unwrap();

        handler.activate(device.as_ref(), "Acoustic").await.unwrap();

        assert_eq!(
            settings::Value::from(Some("Acoustic")),
            current_value(device.as_ref(), SettingId::PresetEqualizerProfile),
        );
    }

    #[tokio::test(start_paused = true)]
    async fn volume_adjustments_win_when_there_is_no_factory_profile() {
        let (device, database, handler) = connect_demo_a3954().await;
        device
            .set_setting_values(vec![(
                SettingId::PresetEqualizerProfile,
                Some("Acoustic").into(),
            )])
            .await
            .unwrap();
        database
            .upsert_quick_preset(
                DeviceModel::SoundcoreA3954,
                QuickPreset {
                    name: "Custom".into(),
                    fields: vec![
                        field(
                            SettingId::PresetEqualizerProfile,
                            settings::Value::OptionalString(None),
                            true,
                        ),
                        field(SettingId::VolumeAdjustments, vec![1i16; 8].into(), true),
                    ],
                },
            )
            .await
            .unwrap();

        handler.activate(device.as_ref(), "Custom").await.unwrap();

        assert_eq!(
            settings::Value::OptionalString(None),
            current_value(device.as_ref(), SettingId::PresetEqualizerProfile),
        );
        assert_eq!(
            settings::Value::from(vec![1i16; 8]),
            current_value(device.as_ref(), SettingId::VolumeAdjustments),
        );
    }

    #[tokio::test(start_paused = true)]
    async fn toggling_on_an_equalizer_field_toggles_off_the_others() {
        let (device, database, handler) = connect_demo_a3954().await;
        device
            .set_setting_values(vec![(
                SettingId::PresetEqualizerProfile,
                Some("Acoustic").into(),
            )])
            .await
            .unwrap();
        handler
            .save(device.as_ref(), "Preset".into())
            .await
            .unwrap();
        handler
            .toggle_field(
                device.as_ref(),
                "Preset".into(),
                SettingId::PresetEqualizerProfile,
                true,
            )
            .await
            .unwrap();
        handler
            .toggle_field(
                device.as_ref(),
                "Preset".into(),
                SettingId::AmbientSoundMode,
                true,
            )
            .await
            .unwrap();

        handler
            .toggle_field(
                device.as_ref(),
                "Preset".into(),
                SettingId::VolumeAdjustments,
                true,
            )
            .await
            .unwrap();

        for (setting_id, expected) in [
            (SettingId::PresetEqualizerProfile, false),
            (SettingId::CustomEqualizerProfile, false),
            (SettingId::VolumeAdjustments, true),
            (SettingId::AmbientSoundMode, true),
        ] {
            assert_eq!(
                Some(expected),
                fetch_field(&database, "Preset", setting_id)
                    .await
                    .map(|field| field.is_enabled),
                "{setting_id:?}",
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn toggling_off_an_equalizer_field_leaves_the_others_alone() {
        let (device, database, handler) = connect_demo_a3954().await;
        handler
            .save(device.as_ref(), "Preset".into())
            .await
            .unwrap();
        handler
            .toggle_field(
                device.as_ref(),
                "Preset".into(),
                SettingId::VolumeAdjustments,
                true,
            )
            .await
            .unwrap();

        handler
            .toggle_field(
                device.as_ref(),
                "Preset".into(),
                SettingId::PresetEqualizerProfile,
                false,
            )
            .await
            .unwrap();

        assert_eq!(
            Some(true),
            fetch_field(&database, "Preset", SettingId::VolumeAdjustments)
                .await
                .map(|field| field.is_enabled),
        );
    }
}
