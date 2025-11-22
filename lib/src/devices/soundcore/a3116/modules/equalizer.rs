use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    connection::RfcommConnection,
    devices::soundcore::common::{
        modules::{
            ModuleCollection,
            equalizer::{EqualizerModuleSettings, EqualizerPreset},
        },
        packet::PacketIOController,
        structures::{EqualizerConfiguration, VolumeAdjustments},
    },
    i18n::fl,
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<EqualizerConfiguration<1, 9, -6, 6, 0>> + Clone + Send + Sync + 'static,
{
    pub async fn add_a3116_equalizer<C>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(state_modifier::EqualizerStateModifier::new(packet_io)),
            module_settings(),
        )
        .await;
    }
}

pub fn module_settings() -> EqualizerModuleSettings<9, 9, -6, 6, 0> {
    EqualizerModuleSettings {
        custom_preset_id: 0xf,
        band_hz: [80, 150, 300, 500, 700, 1000, 5000, 8000, 12000],
        presets: vec![
            EqualizerPreset {
                name: "BassUp",
                localized_name: || fl!("bass-up"),
                id: 0,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "BassOff",
                localized_name: || fl!("bass-off"),
                id: 1,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "Voice",
                localized_name: || fl!("voice"),
                id: 2,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "Heavy",
                localized_name: || fl!("heavy"),
                id: 3,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "Classic",
                localized_name: || fl!("classic"),
                id: 4,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "Original",
                localized_name: || fl!("original"),
                id: 5,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0]),
            },
        ],
    }
}
