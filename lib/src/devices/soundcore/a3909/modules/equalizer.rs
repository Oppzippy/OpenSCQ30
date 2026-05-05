use std::sync::{Arc, LazyLock};

use itertools::Itertools;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    connection::RfcommConnection,
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
    storage::OpenSCQ30Database,
};

impl<T> ModuleCollection<T>
where
    T: Has<a3909::structures::EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    pub async fn add_a3909_equalizer<C>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.add_equalizer(
            packet_io,
            database,
            device_model,
            change_notify,
            EqualizerModuleSettings {
                custom_preset_id: 0xfefe,
                band_hz: [100, 200, 400, 800, 1600, 3200, 6400, 12800],
                presets: PRESETS.clone(),
            },
        )
        .await;
    }
}

pub static PRESETS: LazyLock<Vec<EqualizerPreset<8, -12, 12, 0>>> = LazyLock::new(|| {
    let common_settings = common::modules::equalizer::common_settings();
    common_settings
        .presets
        .into_iter()
        .map(|preset| EqualizerPreset::<8, -12, 12, 0> {
            id: preset.id,
            name: preset.name,
            localized_name: preset.localized_name,
            volume_adjustments: VolumeAdjustments::new(
                preset
                    .volume_adjustments
                    .adjustments()
                    .into_iter()
                    .map(|db| db / 10)
                    .collect_array()
                    .expect("we didn't filter the iterator, so the output will have the same size"),
            ),
        })
        .collect()
});
