use std::{
    collections::{hash_map::Entry, HashMap},
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
    quick_presets: HashMap<Uuid, HashMap<String, QuickPreset>>,
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
    pub fn set_custom_profile(&mut self, name: String, profile: CustomEqualizerProfile) {
        // If multiple profiles with the same volume adjustments existed, it would be ambiguous which should be selected,
        // since the selection is determined only by volume adjustments.
        self.equalizer_custom_profiles
            .retain(|_name, p| p.volume_adjustments() != profile.volume_adjustments());
        self.equalizer_custom_profiles.insert(name, profile);
    }

    pub fn remove_custom_profile(&mut self, name: &str) {
        self.equalizer_custom_profiles.remove(name);
    }

    pub fn custom_profiles(&self) -> &HashMap<String, CustomEqualizerProfile> {
        &self.equalizer_custom_profiles
    }

    pub fn quick_presets(&self, device_service_uuid: Uuid) -> &HashMap<String, QuickPreset> {
        static EMPTY_HASHMAP: LazyLock<HashMap<String, QuickPreset>> = LazyLock::new(HashMap::new);
        self.quick_presets
            .get(&device_service_uuid)
            .unwrap_or(&EMPTY_HASHMAP)
    }

    pub fn set_quick_preset(
        &mut self,
        device_service_uuid: Uuid,
        name: impl Into<String>,
        quick_preset: QuickPreset,
    ) {
        let name = name.into();
        match self.quick_presets.entry(device_service_uuid) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(name, quick_preset);
            }
            Entry::Vacant(entry) => {
                let device_quick_presets = HashMap::from([(name, quick_preset)]);
                entry.insert(device_quick_presets);
            }
        }
    }

    pub fn remove_quick_preset(&mut self, device_service_uuid: Uuid, name: &str) {
        if let Some(device_quick_presets) = self.quick_presets.get_mut(&device_service_uuid) {
            device_quick_presets.remove(name);
        }
    }
}
