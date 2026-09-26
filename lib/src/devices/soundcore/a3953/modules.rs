use openscq30_lib_has::Has;

use crate::devices::soundcore::common::{
    self, device::SoundcoreDeviceBuilder, modules::equalizer::EqualizerModuleSettings,
    structures::TwsStatus,
};

use super::structures::{
    AmbientSoundPrompt, IsHearIdInitialized, PressSensitivity, SoundModes, SpatialAudio,
};

mod equalizer;
mod misc_toggles;
mod press_sensitivity;
mod sound_modes;
mod spatial_audio;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<TwsStatus>
        + Has<common::structures::CommonEqualizerConfiguration<2, 10>>
        + Has<common::structures::CustomHearId<2, 10>>
        + Has<IsHearIdInitialized>
        + Send
        + Sync
        + Clone
        + 'static,
{
    pub async fn a3953_equalizer<const VISIBLE_BANDS: usize, const PRESET_BANDS: usize>(
        &mut self,
        settings: EqualizerModuleSettings<VISIBLE_BANDS, PRESET_BANDS, -120, 134, 1>,
    ) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();

        self.module_collection()
            .add_a3953_equalizer(packet_io, database, device_model, change_notify, settings)
            .await;
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3953_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3953_sound_modes(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<PressSensitivity> + Send + Sync + Clone + 'static,
{
    pub fn a3953_press_sensitivity(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3953_press_sensitivity(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<AmbientSoundPrompt> + Send + Sync + Clone + 'static,
{
    pub fn a3953_misc_toggles(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3953_misc_toggles(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<SpatialAudio> + Send + Sync + Clone + 'static,
{
    pub fn a3953_spatial_audio(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3953_spatial_audio(packet_io_controller);
    }
}
