use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    connection::RfcommConnection,
    devices::soundcore::common::{
        modules::ModuleCollection,
        packet::PacketIOController,
        structures::{CustomHearId, EqualizerConfiguration},
    },
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<EqualizerConfiguration<1, 10>>
        + Has<CustomHearId<2, 10>>
        + Clone
        + Send
        + Sync
        + 'static,
{
    pub async fn add_a3040_equalizer<C>(
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
            Box::new(state_modifier::EqualizerWithCustomHearIdStateModifier::new(
                packet_io,
            )),
        )
        .await;
    }
}
