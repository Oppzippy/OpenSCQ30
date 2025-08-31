use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::common::{
        packet::{outbound::SetSoundModePacket, packet_io_controller::PacketIOController},
        state_modifier::StateModifier,
        structures::{AmbientSoundMode, SoundModes},
    },
};

pub struct SoundModesStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> SoundModesStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for SoundModesStateModifier<ConnectionType>
where
    T: Has<SoundModes> + Clone + Send + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let sound_modes = *state_sender.borrow().get();
        let target_sound_modes = target_state.get();
        if &sound_modes == target_sound_modes {
            return Ok(());
        }

        // It will bug and put us in noise canceling mode without changing the ambient sound mode id if we change the
        // noise canceling mode with the ambient sound mode being normal or transparency. To work around this, we must
        // set the ambient sound mode to Noise Canceling, and then change it back.
        let needs_noise_canceling = sound_modes.ambient_sound_mode
            != AmbientSoundMode::NoiseCanceling
            && sound_modes.noise_canceling_mode != target_sound_modes.noise_canceling_mode;
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
                .send_with_response(&SetSoundModePacket(new_sound_modes).into())
                .await?;
            state_sender.send_modify(|state| *state.get_mut() = new_sound_modes);
        }

        {
            let new_sound_modes = SoundModes {
                ambient_sound_mode: if needs_noise_canceling {
                    AmbientSoundMode::NoiseCanceling
                } else {
                    target_sound_modes.ambient_sound_mode
                },
                noise_canceling_mode: target_sound_modes.noise_canceling_mode,
                transparency_mode: target_sound_modes.transparency_mode,
                custom_noise_canceling: target_sound_modes.custom_noise_canceling,
            };
            // If we need to temporarily be in noise canceling mode to work around the bug, set all fields besides
            // ambient_sound_mode. Otherwise, we set all fields in one go.
            self.packet_io
                .send_with_response(&SetSoundModePacket(new_sound_modes).into())
                .await?;
            state_sender.send_modify(|state| *state.get_mut() = new_sound_modes);
        }

        // Switch to the target sound mode if we didn't do it in the previous step.
        // If the target sound mode is noise canceling, we already set it to that, so no change needed.
        if needs_ambient_sound_mode_revert {
            self.packet_io
                .send_with_response(&SetSoundModePacket(*target_sound_modes).into())
                .await?;
            state_sender.send_modify(|state| *state.get_mut() = *target_sound_modes);
        }
        Ok(())
    }
}
