use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::common::{
        self,
        modules::{
            ModuleCollection,
            equalizer::{EqualizerModuleSettings, EqualizerPreset},
        },
        packet::PacketIOController,
        structures::{CommonEqualizerConfiguration, CustomHearId, VolumeAdjustments},
    },
    i18n::fl,
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<CommonEqualizerConfiguration<1, 10>>
        + Has<CustomHearId<1, 10>>
        + Clone
        + Send
        + Sync
        + 'static,
{
    pub async fn add_a3062_equalizer(
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
            Box::new(state_modifier::EqualizerWithCustomHearIdStateModifier::new(
                packet_io,
            )),
            equalizer_settings(),
        )
        .await;
    }
}

fn equalizer_settings() -> EqualizerModuleSettings<8, 10, -120, 134, 1> {
    common::modules::equalizer::common_settings_with_presets(vec![
        EqualizerPreset {
            name: "SoundcoreSignature",
            localized_name: || fl!("soundcore-signature"),
            id: 0,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0, -120]),
        },
        EqualizerPreset {
            name: "Acoustic",
            localized_name: || fl!("acoustic"),
            id: 1,
            volume_adjustments: VolumeAdjustments::new([40, 10, 20, 20, 40, 40, 40, 20, 0, -120]),
        },
        EqualizerPreset {
            name: "BassReducer",
            localized_name: || fl!("bass-reducer"),
            id: 3,
            volume_adjustments: VolumeAdjustments::new([-40, -30, -10, 0, 0, 0, 0, 0, 0, -120]),
        },
        EqualizerPreset {
            name: "Classical",
            localized_name: || fl!("classical"),
            id: 4,
            volume_adjustments: VolumeAdjustments::new([30, 30, -20, -20, 0, 20, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Podcast",
            localized_name: || fl!("podcast"),
            id: 5,
            volume_adjustments: VolumeAdjustments::new([-30, 20, 40, 40, 30, 20, 0, -20, 0, -120]),
        },
        EqualizerPreset {
            name: "Dance",
            localized_name: || fl!("dance"),
            id: 6,
            volume_adjustments: VolumeAdjustments::new([
                20, -30, -10, 10, 20, 20, 10, -30, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "Deep",
            localized_name: || fl!("deep"),
            id: 7,
            volume_adjustments: VolumeAdjustments::new([
                20, 10, 30, 30, 20, -20, -40, -50, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "Electronic",
            localized_name: || fl!("electronic"),
            id: 8,
            volume_adjustments: VolumeAdjustments::new([30, 20, -20, 20, 10, 20, 30, 30, 0, -120]),
        },
        EqualizerPreset {
            name: "Flat",
            localized_name: || fl!("flat"),
            id: 9,
            volume_adjustments: VolumeAdjustments::new([-20, -20, -10, 0, 0, 0, -20, -20, 0, -120]),
        },
        EqualizerPreset {
            name: "HipHop",
            localized_name: || fl!("hip-hop"),
            id: 10,
            volume_adjustments: VolumeAdjustments::new([
                20, 30, -10, -10, 20, -10, 20, 30, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "Jazz",
            localized_name: || fl!("jazz"),
            id: 11,
            volume_adjustments: VolumeAdjustments::new([20, 20, -20, -20, 0, 20, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Latin",
            localized_name: || fl!("latin"),
            id: 12,
            volume_adjustments: VolumeAdjustments::new([0, 0, -20, -20, -20, 0, 30, 50, 0, -120]),
        },
        EqualizerPreset {
            name: "Lounge",
            localized_name: || fl!("lounge"),
            id: 13,
            volume_adjustments: VolumeAdjustments::new([-10, 20, 40, 30, 0, -20, 20, 10, 0, -120]),
        },
        EqualizerPreset {
            name: "Piano",
            localized_name: || fl!("piano"),
            id: 14,
            volume_adjustments: VolumeAdjustments::new([0, 30, 30, 20, 40, 50, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Pop",
            localized_name: || fl!("pop"),
            id: 15,
            volume_adjustments: VolumeAdjustments::new([
                -10, 10, 30, 30, 10, -10, -20, -30, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "RnB",
            localized_name: || fl!("rnb"),
            id: 16,
            volume_adjustments: VolumeAdjustments::new([60, 20, -20, -20, 20, 30, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Rock",
            localized_name: || fl!("rock"),
            id: 17,
            volume_adjustments: VolumeAdjustments::new([30, 20, -10, -10, 10, 30, 40, 50, 0, -120]),
        },
        EqualizerPreset {
            name: "SmallSpeakers",
            localized_name: || fl!("small-speakers"),
            id: 18,
            volume_adjustments: VolumeAdjustments::new([
                40, 30, 10, 0, -20, -30, -40, -40, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "SpokenWord",
            localized_name: || fl!("spoken-word"),
            id: 19,
            volume_adjustments: VolumeAdjustments::new([-30, -20, 10, 20, 20, 10, 0, -30, 0, -120]),
        },
        EqualizerPreset {
            name: "TrebleBooster",
            localized_name: || fl!("treble-booster"),
            id: 20,
            volume_adjustments: VolumeAdjustments::new([
                -20, -20, -20, -10, 10, 20, 20, 40, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "TrebleReducer",
            localized_name: || fl!("treble-reducer"),
            id: 21,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60, 0, -120]),
        },
        EqualizerPreset {
            name: "BassBooster",
            localized_name: || fl!("bass-booster"),
            id: 0x7e7e, // yes, this number is correct. no idea why it's this instead of the usual 2.
            volume_adjustments: VolumeAdjustments::new([40, 30, 10, 0, 0, 0, 0, 0, 0, -120]),
        },
    ])
}
