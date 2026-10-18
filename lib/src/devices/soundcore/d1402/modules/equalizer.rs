use std::sync::Arc;

use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    devices::soundcore::{
        a3062,
        common::{
            modules::ModuleCollection,
            packet::PacketIOController,
            structures::{CommonEqualizerConfiguration, CustomHearId},
        },
        d1402,
    },
    storage::OpenSCQ30Database,
};

mod state_modifier;

impl<T> ModuleCollection<T>
where
    T: Has<CommonEqualizerConfiguration<1, 10>>
        + Has<CustomHearId<1, 10>>
        + Has<d1402::structures::EqualizerUnknownBytes>
        + Clone
        + Send
        + Sync
        + 'static,
{
    pub async fn add_d1402_equalizer(
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
            // same presets as the Space One Pro
            a3062::modules::equalizer::equalizer_settings(),
        )
        .await;
    }
}
