use openscq30_lib_has::Has;

use crate::devices::soundcore::{
    common::{
        self,
        device::SoundcoreDeviceBuilder,
        structures::{AgeRange, Ldac, SurroundSound},
    },
    d1402,
};

mod button_configuration;
mod equalizer;
mod flag;
mod ldac_and_three_d_sound;
mod sound_modes;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<d1402::structures::SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn d1402_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1402_sound_modes(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<common::structures::CommonEqualizerConfiguration<1, 10>>
        + Has<common::structures::CustomHearId<1, 10>>
        + Has<d1402::structures::EqualizerUnknownByte>
        + Has<AgeRange>
        + Send
        + Sync
        + Clone
        + 'static,
{
    pub async fn d1402_equalizer(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        self.module_collection()
            .add_d1402_equalizer(packet_io_controller, database, device_model, change_notify)
            .await;
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<SurroundSound> + Has<Ldac> + Send + Sync + Clone + 'static,
{
    /// 3D sound is exposed as surround sound.
    pub fn d1402_ldac_and_three_d_sound(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1402_ldac_and_three_d_sound(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<d1402::structures::ButtonConfiguration> + Send + Sync + Clone + 'static,
{
    pub fn d1402_button_configuration(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1402_button_configuration(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<d1402::structures::AmbientSoundModeVoicePrompt> + Send + Sync + Clone + 'static,
{
    pub fn d1402_ambient_sound_mode_voice_prompt(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_d1402_ambient_sound_mode_voice_prompt(packet_io_controller);
    }
}
