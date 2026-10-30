use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::common::{
        modules::{self, ModuleCollection},
        packet::PacketIOController,
        structures::CommonEqualizerConfiguration,
    },
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<CommonEqualizerConfiguration<2, 10>> + Clone + Send + Sync + 'static,
{
    pub async fn add_a3388_equalizer(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
        packet_io: Arc<PacketIOController>,
    ) {
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(state_modifier::EqualizerStateModifier::new(packet_io)),
            modules::equalizer::common_settings_type_2(),
        )
        .await;
    }
}
