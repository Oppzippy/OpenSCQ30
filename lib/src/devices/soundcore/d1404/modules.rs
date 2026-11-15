use openscq30_lib_has::Has;

use crate::devices::soundcore::{
    common::{
        device::SoundcoreDeviceBuilder,
        structures::{Ldac, SurroundSound},
    },
    d1404,
};

mod button_configuration;
mod equalizer;
mod flag;
mod ldac_and_three_d_sound;
mod sound_modes;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<d1404::structures::SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn d1404_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1404_sound_modes(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<d1404::structures::D1404EqualizerConfiguration> + Send + Sync + Clone + 'static,
{
    pub async fn d1404_equalizer(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        self.module_collection()
            .add_d1404_equalizer(packet_io_controller, database, device_model, change_notify)
            .await;
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<d1404::structures::ButtonConfiguration>
        + Has<d1404::structures::MfbButtonConfiguration>
        + Send
        + Sync
        + Clone
        + 'static,
{
    pub fn d1404_button_configuration(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1404_button_configuration(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<SurroundSound> + Has<Ldac> + Send + Sync + Clone + 'static,
{
    /// 3D sound is exposed as surround sound.
    pub fn d1404_ldac_and_three_d_sound(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1404_ldac_and_three_d_sound(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<d1404::structures::AmbientSoundModeVoicePrompt> + Send + Sync + Clone + 'static,
{
    pub fn d1404_ambient_sound_mode_voice_prompt(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1404_ambient_sound_mode_voice_prompt(packet_io_controller);
    }
}
