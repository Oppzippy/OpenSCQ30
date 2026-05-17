use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::common::{
        modules::{self, ModuleCollection},
        packet::PacketIOController,
        structures::{CommonEqualizerConfiguration, CustomHearId},
    },
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
    pub async fn add_a3035_equalizer(
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
            modules::equalizer::common_settings(),
        )
        .await;
    }
}
