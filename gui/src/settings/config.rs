use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Write,
    sync::{Arc, LazyLock},
};

use openscq30_lib::devices::standard::structures::{
    AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, PresetEqualizerProfile,
    TransparencyMode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::custom_equalizer_profile::CustomEqualizerProfile;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    equalizer_custom_profiles: HashMap<String, CustomEqualizerProfile>,
    // model number -> (quick preset name -> quick preset settings)
    quick_presets: HashMap<String, HashMap<String, QuickPreset>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, Hash)]
pub struct QuickPreset {
    pub ambient_sound_mode: Option<AmbientSoundMode>,
    pub transparency_mode: Option<TransparencyMode>,
    pub noise_canceling_mode: Option<NoiseCancelingMode>,
    pub custom_noise_canceling: Option<CustomNoiseCanceling>,
    pub equalizer_profile: Option<PresetOrCustomEqualizerProfile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PresetOrCustomEqualizerProfile {
    Preset(PresetEqualizerProfile),
    Custom(Arc<str>),
}

impl Config {
    /// Set a single profile. This iterates all existing profiles for validation, so use insert_custom_profiles when inserting many.
    pub fn set_custom_profile(&mut self, name: String, profile: CustomEqualizerProfile) {
        // If multiple profiles with the same volume adjustments existed, it would be ambiguous which should be selected,
        // since the selection is determined only by volume adjustments.
        self.equalizer_custom_profiles
            .retain(|_name, p| p.volume_adjustments() != profile.volume_adjustments());
        self.equalizer_custom_profiles.insert(name, profile);
    }

    /// Like set_custom_profile except optimized for inserting multiple profiles
    pub fn insert_custom_profiles(
        &mut self,
        profiles: impl IntoIterator<Item = (String, CustomEqualizerProfile)>,
        overwrite: bool,
    ) {
        let profiles = profiles.into_iter();
        let unique_values: HashMap<CustomEqualizerProfile, String> = self
            .equalizer_custom_profiles
            .iter()
            .map(|(key, value)| (value.to_owned(), key.to_owned()))
            .collect();
        for (mut name, profile) in profiles {
            let existing_name_from_values = unique_values.get(&profile);
            if !overwrite {
                // If a profile exists with the same values, there's nothing we can do to not overwrite
                if existing_name_from_values.is_some() {
                    continue;
                }
                if self.equalizer_custom_profiles.contains_key(&name) {
                    match self.find_name_for_duplicate(name) {
                        Some(new_name) => name = new_name,
                        None => continue,
                    }
                }
            }
            self.equalizer_custom_profiles.insert(name, profile);
            existing_name_from_values.into_iter().for_each(|name| {
                self.equalizer_custom_profiles.remove(name);
            });
        }
    }

    fn find_name_for_duplicate(&self, mut name: String) -> Option<String> {
        let original_name_len = name.len();
        for i in 2..1000 {
            write!(name, " ({i})").unwrap();
            if !self.equalizer_custom_profiles.contains_key(&name) {
                return Some(name);
            }
            name.truncate(original_name_len);
        }
        None
    }

    pub fn remove_custom_profile(&mut self, name: &str) {
        self.equalizer_custom_profiles.remove(name);
    }

    pub fn custom_profiles(&self) -> &HashMap<String, CustomEqualizerProfile> {
        &self.equalizer_custom_profiles
    }

    pub fn quick_presets(&self, device_model: &str) -> &HashMap<String, QuickPreset> {
        static EMPTY_HASHMAP: LazyLock<HashMap<String, QuickPreset>> = LazyLock::new(HashMap::new);
        self.quick_presets
            .get(device_model)
            .unwrap_or(&EMPTY_HASHMAP)
    }

    pub fn set_quick_preset(
        &mut self,
        device_model: String,
        name: String,
        quick_preset: QuickPreset,
    ) {
        match self.quick_presets.entry(device_model) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(name, quick_preset);
            }
            Entry::Vacant(entry) => {
                let device_quick_presets = HashMap::from([(name, quick_preset)]);
                entry.insert(device_quick_presets);
            }
        }
    }

    pub fn remove_quick_preset(&mut self, device_model: String, name: &str) {
        if let Some(device_quick_presets) = self.quick_presets.get_mut(&device_model) {
            device_quick_presets.remove(name);
        }
    }

    pub fn migrate_service_uuid_to_device_model(
        &mut self,
        service_uuid: Uuid,
        device_model: String,
    ) {
        let Some(service_presets) = self.quick_presets.remove(&service_uuid.to_string()) else {
            return;
        };
        let model_presets = match self.quick_presets.entry(device_model) {
            Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            Entry::Vacant(vacant_entry) => vacant_entry.insert(HashMap::new()),
        };
        model_presets.extend(service_presets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn test_migrate_service_uuids_with_none_target() {
        let mut config = Config::default();
        let uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        let mut uuid_map = HashMap::new();
        uuid_map.insert("test1".to_string(), QuickPreset::default());
        config.quick_presets.insert(uuid.to_string(), uuid_map);

        config.migrate_service_uuid_to_device_model(uuid, "0123".to_string());
        assert_eq!(config.quick_presets.get("0123").unwrap().len(), 1);
        assert_eq!(config.quick_presets.get(&uuid.to_string()), None);
    }

    #[test]
    fn test_migrate_service_uuids_with_some_target() {
        let mut config = Config::default();
        let uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

        let mut uuid_map = HashMap::new();
        uuid_map.insert("test1".to_string(), QuickPreset::default());
        config.quick_presets.insert(uuid.to_string(), uuid_map);

        let mut model_map = HashMap::new();
        model_map.insert("test2".to_string(), QuickPreset::default());
        config.quick_presets.insert("0123".to_string(), model_map);

        config.migrate_service_uuid_to_device_model(uuid, "0123".to_string());
        assert_eq!(config.quick_presets.get("0123").unwrap().len(), 2);
        assert_eq!(config.quick_presets.get(&uuid.to_string()), None);
    }
}
