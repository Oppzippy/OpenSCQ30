use crate::devices::soundcore::{
    a3876::state::A3876State,
    common::{device::SoundcoreDeviceBuilder, modules::equalizer::EqualizerModuleSettings},
};

mod colorful_lights;
mod equalizer;
mod volume_balance;

impl SoundcoreDeviceBuilder<A3876State> {
    pub async fn a3876_equalizer<const VISIBLE_BANDS: usize, const PRESET_BANDS: usize>(
        &mut self,
        settings: EqualizerModuleSettings<VISIBLE_BANDS, PRESET_BANDS, -120, 134, 1>,
    ) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();

        self.module_collection()
            .add_a3876_equalizer(packet_io, database, device_model, change_notify, settings)
            .await;
    }

    pub fn a3876_volume_balance(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3876_volume_balance(packet_io_controller);
    }

    pub fn a3876_colorful_lights(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3876_colorful_lights(packet_io_controller);
    }
}
