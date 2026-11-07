use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::{
        a3134::structures::{self, A3134EqualizerConfiguration, CUSTOM_EQUALIZER_PRESET_ID},
        common::{
            modules::{
                ModuleCollection,
                equalizer::{EqualizerModuleSettings, InvisibleBandsMode},
            },
            packet::PacketIOController,
        },
    },
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<A3134EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    pub async fn add_a3134_equalizer(
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
            module_settings(),
        )
        .await;
    }
}

fn module_settings() -> EqualizerModuleSettings<9, 9, -60, 60, 1> {
    EqualizerModuleSettings {
        custom_preset_id: Some(CUSTOM_EQUALIZER_PRESET_ID.into()),
        band_hz: [80, 150, 300, 600, 1200, 2500, 5000, 9000, 13000],
        invisible_bands_mode: InvisibleBandsMode::Fixed(Vec::new()),
        preset_min_volume: -60,
        preset_max_volume: 60,
        presets: structures::equalizer_presets().to_vec(),
    }
}
