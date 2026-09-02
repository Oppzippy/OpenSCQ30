use crate::devices::soundcore::{common::device::SoundcoreDeviceBuilder, d1301::state::D1301State};

mod alarms;
mod auto_stop_timer;
mod auto_switch_once_asleep;
mod flags;
mod listening_mode;

macro_rules! flag {
    ($name: ident) => {
        paste::paste! {
            pub fn [< d1301_ $name >](&mut self) {
                let packet_io = self.packet_io_controller().clone();
                self.module_collection()
                    .[< add_d1301_ $name >](packet_io);
            }
        }
    };
}

impl SoundcoreDeviceBuilder<D1301State> {
    flag!(auto_power_off_prompt);
    flag!(listening_mode_prompt);
    flag!(noise_canceling);
    flag!(incoming_calls_during_bluetooth_mode);
    flag!(tap_controls_disabled);
    flag!(noise_canceling_prompt);

    pub fn d1301_alarms(&mut self) {
        self.module_collection().add_d1301_alarms();
    }

    pub fn d1301_auto_stop_timer(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1301_auto_stop_timer(packet_io);
    }

    pub fn d1301_auto_switch_once_asleep(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1301_auto_switch_once_asleep(packet_io);
    }

    pub fn d1301_listening_mode(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_d1301_listening_mode(packet_io);
    }
}
