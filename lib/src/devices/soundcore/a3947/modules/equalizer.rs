use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    connection::RfcommConnection,
    devices::soundcore::{
        a3947,
        common::{
            self,
            modules::{ModuleCollection, equalizer::COMMON_EQUALIZER_MODULE_SETTINGS},
            packet::PacketIOController,
            structures::TwsStatus,
        },
    },
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<common::structures::EqualizerConfiguration<2, 10>>
        + Has<a3947::structures::HearId<2, 10>>
        + Has<TwsStatus>
        + Clone
        + Send
        + Sync
        + 'static,
{
    pub async fn add_a3947_equalizer<C>(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
        packet_io: Arc<PacketIOController<C>>,
    ) where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(state_modifier::EqualizerStateModifier::new(packet_io)),
            COMMON_EQUALIZER_MODULE_SETTINGS,
        )
        .await;
    }
}
