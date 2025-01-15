use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::connection::Connection,
    devices::standard::{
        packets::outbound::SetSoundModePacket,
        state_modifier::StateModifier,
        structures::{AmbientSoundMode, SoundModes},
    },
    futures::Futures,
    soundcore_device::device::packet_io_controller::PacketIOController,
};

pub struct SoundModesStateModifier<ConnectionType: Connection, FuturesType: Futures> {
    packet_io: Arc<PacketIOController<ConnectionType, FuturesType>>,
}

impl<ConnectionType: Connection, FuturesType: Futures>
    SoundModesStateModifier<ConnectionType, FuturesType>
{
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType, FuturesType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait(?Send)]
impl<ConnectionType, FuturesType, T> StateModifier<T>
    for SoundModesStateModifier<ConnectionType, FuturesType>
where
    T: AsMut<SoundModes> + AsRef<SoundModes> + Clone,
    ConnectionType: Connection,
    FuturesType: Futures,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let sound_modes = state_sender.borrow().as_ref().clone();
        let target_sound_modes = target_state.as_ref();
        if &sound_modes == target_sound_modes {
            return Ok(());
        }

        // It will bug and put us in noise canceling mode without changing the ambient sound mode id if we change the
        // noise canceling mode with the ambient sound mode being normal or transparency. To work around this, we must
        // set the ambient sound mode to Noise Canceling, and then change it back.
        let needs_noise_canceling = sound_modes.ambient_sound_mode
            != AmbientSoundMode::NoiseCanceling
            && sound_modes.noise_canceling_mode != sound_modes.noise_canceling_mode;
        let needs_ambient_sound_mode_revert = needs_noise_canceling
            && sound_modes.ambient_sound_mode != AmbientSoundMode::NoiseCanceling;

        if needs_noise_canceling {
            let new_sound_modes = SoundModes {
                ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
                noise_canceling_mode: sound_modes.noise_canceling_mode,
                transparency_mode: sound_modes.transparency_mode,
                custom_noise_canceling: sound_modes.custom_noise_canceling,
            };
            self.packet_io
                .send(&SetSoundModePacket(new_sound_modes).into())
                .await?;
            state_sender.send_modify(|state| *state.as_mut() = new_sound_modes);
        }

        {
            let new_sound_modes = SoundModes {
                ambient_sound_mode: if needs_noise_canceling {
                    AmbientSoundMode::NoiseCanceling
                } else {
                    sound_modes.ambient_sound_mode
                },
                noise_canceling_mode: target_sound_modes.noise_canceling_mode,
                transparency_mode: target_sound_modes.transparency_mode,
                custom_noise_canceling: target_sound_modes.custom_noise_canceling,
            };
            // If we need to temporarily be in noise canceling mode to work around the bug, set all fields besides
            // ambient_sound_mode. Otherwise, we set all fields in one go.
            self.packet_io
                .send(&SetSoundModePacket(new_sound_modes).into())
                .await?;
            state_sender.send_modify(|state| *state.as_mut() = new_sound_modes);
        }

        // Switch to the target sound mode if we didn't do it in the previous step.
        // If the target sound mode is noise canceling, we already set it to that, so no change needed.
        if needs_ambient_sound_mode_revert {
            self.packet_io
                .send(&SetSoundModePacket(*target_sound_modes).into())
                .await?;
            state_sender.send_modify(|state| *state.as_mut() = *target_sound_modes);
        }
        Ok(())
    }
}
