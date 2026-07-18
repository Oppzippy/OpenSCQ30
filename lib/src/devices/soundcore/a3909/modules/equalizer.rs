use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::{
        a3909,
        common::{
            self,
            modules::{
                ModuleCollection,
                equalizer::{EqualizerModuleSettings, EqualizerPreset},
            },
            packet::PacketIOController,
            structures::VolumeAdjustments,
        },
    },
    i18n::fl,
    storage::OpenSCQ30Database,
};

impl<T> ModuleCollection<T>
where
    T: Has<a3909::structures::EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    pub async fn add_a3909_equalizer(
        &mut self,
        packet_io: Arc<PacketIOController>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) {
        self.add_equalizer(
            packet_io,
            database,
            device_model,
            change_notify,
            equalizer_settings(),
        )
        .await;
    }
}

pub fn equalizer_settings() -> EqualizerModuleSettings<8, 8, -12, 12, 0> {
    common::modules::equalizer::common_settings_with_presets(vec![
        EqualizerPreset {
            name: "SoundcoreSignature",
            localized_name: || fl!("soundcore-signature"),
            id: 0,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0]),
        },
        EqualizerPreset {
            name: "Acoustic",
            localized_name: || fl!("acoustic"),
            id: 1,
            volume_adjustments: VolumeAdjustments::new([4, 1, 2, 2, 4, 4, 4, 2]),
        },
        EqualizerPreset {
            name: "BassBooster",
            localized_name: || fl!("bass-booster"),
            id: 2,
            volume_adjustments: VolumeAdjustments::new([4, 3, 1, 0, 0, 0, 0, 0]),
        },
        EqualizerPreset {
            name: "BassReducer",
            localized_name: || fl!("bass-reducer"),
            id: 3,
            volume_adjustments: VolumeAdjustments::new([-4, -3, -1, 0, 0, 0, 0, 0]),
        },
        EqualizerPreset {
            name: "Classical",
            localized_name: || fl!("classical"),
            id: 4,
            volume_adjustments: VolumeAdjustments::new([3, 3, -2, -2, 0, 2, 3, 4]),
        },
        EqualizerPreset {
            name: "Podcast",
            localized_name: || fl!("podcast"),
            id: 5,
            volume_adjustments: VolumeAdjustments::new([-3, 2, 4, 4, 3, 2, 0, -2]),
        },
        EqualizerPreset {
            name: "Dance",
            localized_name: || fl!("dance"),
            id: 6,
            volume_adjustments: VolumeAdjustments::new([2, -3, -1, 1, 2, 2, 1, -3]),
        },
        EqualizerPreset {
            name: "Deep",
            localized_name: || fl!("deep"),
            id: 7,
            volume_adjustments: VolumeAdjustments::new([2, 1, 3, 3, 2, -2, -4, -5]),
        },
        EqualizerPreset {
            name: "Electronic",
            localized_name: || fl!("electronic"),
            id: 8,
            volume_adjustments: VolumeAdjustments::new([3, 2, -2, 2, 1, 2, 3, 3]),
        },
        EqualizerPreset {
            name: "Flat",
            localized_name: || fl!("flat"),
            id: 9,
            volume_adjustments: VolumeAdjustments::new([-2, -2, -1, 0, 0, 0, -2, -2]),
        },
        EqualizerPreset {
            name: "HipHop",
            localized_name: || fl!("hip-hop"),
            id: 10,
            volume_adjustments: VolumeAdjustments::new([2, 3, -1, -1, 2, -1, 2, 3]),
        },
        EqualizerPreset {
            name: "Jazz",
            localized_name: || fl!("jazz"),
            id: 11,
            volume_adjustments: VolumeAdjustments::new([2, 2, -2, -2, 0, 2, 3, 4]),
        },
        EqualizerPreset {
            name: "Latin",
            localized_name: || fl!("latin"),
            id: 12,
            volume_adjustments: VolumeAdjustments::new([0, 0, -2, -2, -2, 0, 3, 5]),
        },
        EqualizerPreset {
            name: "Lounge",
            localized_name: || fl!("lounge"),
            id: 13,
            volume_adjustments: VolumeAdjustments::new([-1, 2, 4, 3, 0, -2, 2, 1]),
        },
        EqualizerPreset {
            name: "Piano",
            localized_name: || fl!("piano"),
            id: 14,
            volume_adjustments: VolumeAdjustments::new([0, 3, 3, 2, 4, 5, 3, 4]),
        },
        EqualizerPreset {
            name: "Pop",
            localized_name: || fl!("pop"),
            id: 15,
            volume_adjustments: VolumeAdjustments::new([-1, 1, 3, 3, 1, -1, -2, -3]),
        },
        EqualizerPreset {
            name: "RnB",
            localized_name: || fl!("rnb"),
            id: 16,
            volume_adjustments: VolumeAdjustments::new([6, 2, -2, -2, 2, 3, 3, 4]),
        },
        EqualizerPreset {
            name: "Rock",
            localized_name: || fl!("rock"),
            id: 17,
            volume_adjustments: VolumeAdjustments::new([3, 2, -1, -1, 1, 3, 3, 3]),
        },
        EqualizerPreset {
            name: "SmallSpeakers",
            localized_name: || fl!("small-speakers"),
            id: 18,
            volume_adjustments: VolumeAdjustments::new([4, 3, 1, 0, -2, -3, -4, -4]),
        },
        EqualizerPreset {
            name: "SpokenWord",
            localized_name: || fl!("spoken-word"),
            id: 19,
            volume_adjustments: VolumeAdjustments::new([-3, -2, 1, 2, 2, 1, 0, -3]),
        },
        EqualizerPreset {
            name: "TrebleBooster",
            localized_name: || fl!("treble-booster"),
            id: 20,
            volume_adjustments: VolumeAdjustments::new([-2, -2, -2, -1, 1, 2, 2, 4]),
        },
        EqualizerPreset {
            name: "TrebleReducer",
            localized_name: || fl!("treble-reducer"),
            id: 21,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, -2, -3, -4, -4, -6]),
        },
    ])
}
