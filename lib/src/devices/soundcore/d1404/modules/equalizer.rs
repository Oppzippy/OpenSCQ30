use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::{
        common::modules::{
            ModuleCollection,
            equalizer::{EqualizerModuleSettings, EqualizerPreset, InvisibleBandsMode},
        },
        common::packet::PacketIOController,
        common::structures::VolumeAdjustments,
        d1404,
    },
    i18n::fl,
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<d1404::structures::D1404EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    pub async fn add_d1404_equalizer(
        &mut self,
        packet_io: Arc<PacketIOController>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) {
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(state_modifier::EqualizerStateModifier::new(packet_io)),
            equalizer_settings(),
        )
        .await;
    }
}

/// Preset equalizer curves. The device only stores the preset id, the app renders the curve
/// from the values below, which approximate the fixed filter sets that the official app
/// writes out (see `d1404::packets::outbound::preset_filters`).
fn equalizer_settings() -> EqualizerModuleSettings<8, 8, -60, 60, 1> {
    EqualizerModuleSettings {
        custom_preset_id: Some(0xfefe),
        band_hz: [100, 200, 400, 800, 1600, 3200, 6400, 12800],
        invisible_bands_mode: InvisibleBandsMode::Fixed(vec![]),
        preset_min_volume: -60,
        preset_max_volume: 60,
        presets: vec![
            EqualizerPreset {
                name: "SoundcoreSignature",
                localized_name: || fl!("soundcore-signature"),
                id: 0,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "VocalBoost",
                localized_name: || fl!("vocal-boost"),
                id: 1,
                volume_adjustments: VolumeAdjustments::new([0, 0, -9, 24, 0, 50, 30, 0]),
            },
            EqualizerPreset {
                name: "BassBooster",
                localized_name: || fl!("bass-booster"),
                id: 2,
                volume_adjustments: VolumeAdjustments::new([60, 20, -15, 0, 0, 15, 0, 0]),
            },
            EqualizerPreset {
                name: "Calming",
                localized_name: || fl!("calming"),
                id: 3,
                volume_adjustments: VolumeAdjustments::new([30, 40, 0, 0, 0, -50, 10, 0]),
            },
        ],
    }
}
