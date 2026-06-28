mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::{
        a3954,
        common::{
            self,
            modules::{ModuleCollection, equalizer::EqualizerModuleSettings},
            packet::PacketIOController,
        },
    },
    storage::OpenSCQ30Database,
};

impl<T> ModuleCollection<T>
where
    T: Has<common::structures::TwsStatus>
        + Has<a3954::structures::SpatialAudio>
        + Clone
        + Send
        + Sync
        + 'static,
{
    pub async fn add_a3954_equalizer<
        const CHANNELS: usize,
        const BANDS: usize,
        const VISIBLE_BANDS: usize,
        const PRESET_BANDS: usize,
        const MIN_VOLUME: i16,
        const MAX_VOLUME: i16,
        const FRACTION_DIGITS: u8,
    >(
        &mut self,
        packet_io: Arc<PacketIOController>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
        settings: EqualizerModuleSettings<
            VISIBLE_BANDS,
            PRESET_BANDS,
            MIN_VOLUME,
            MAX_VOLUME,
            FRACTION_DIGITS,
        >,
    ) where
        T: Has<
                common::structures::EqualizerConfiguration<
                    CHANNELS,
                    BANDS,
                    MIN_VOLUME,
                    MAX_VOLUME,
                    FRACTION_DIGITS,
                >,
            > + Has<common::structures::CustomHearId<CHANNELS, BANDS>>,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(state_modifier::EqualizerStateModifier::new(packet_io)),
            settings,
        )
        .await;
    }
}
