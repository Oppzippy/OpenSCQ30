use openscq30_lib_has::Has;

use crate::devices::soundcore::common::{
    device::SoundcoreDeviceBuilder, structures::CommonEqualizerConfiguration,
};

mod equalizer;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<CommonEqualizerConfiguration<2, 10>> + Send + Sync + Clone + 'static,
{
    pub async fn a3388_equalizer(&mut self) {
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        let packet_io = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3388_equalizer(database, device_model, change_notify, packet_io)
            .await;
    }
}
