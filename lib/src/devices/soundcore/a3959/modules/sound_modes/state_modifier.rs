use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3959,
        common::{self, packet::PacketIOController, state_modifier::StateModifier},
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
    T: Has<a3959::structures::SoundModes> + Clone + Send + Sync,
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

        let change_plan = create_change_plan(sound_modes, *target_sound_modes);
        for step in change_plan {
            self.packet_io
                .send_with_response(
                    &a3959::packets::outbound::A3959SetSoundModes { sound_modes: step }.into(),
                )
                .await?;
            state_sender.send_modify(|state| *state.get_mut() = step);
        }

        Ok(())
    }
}

/// Creates a plan to get from the current state to the target state without making invalid state
/// transitions. For example, changing the manual noise canceling level while in normal mode is not
/// allowed.
fn create_change_plan(
    from: a3959::structures::SoundModes,
    to: a3959::structures::SoundModes,
) -> Vec<a3959::structures::SoundModes> {
    let mut sequence = Vec::new();
    let mut current = from;

    // Go in order of most to least dependencies so that things are left in their desired state
    if current.adaptive_noise_canceling != to.adaptive_noise_canceling
        || current.noise_canceling_adaptive_sensitivity_level
            != to.noise_canceling_adaptive_sensitivity_level
    {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(current);
        }
        if current.noise_canceling_mode != a3959::structures::NoiseCancelingMode::Adaptive {
            current.noise_canceling_mode = a3959::structures::NoiseCancelingMode::Adaptive;
            sequence.push(current);
        }
        current.adaptive_noise_canceling = to.adaptive_noise_canceling;
        current.noise_canceling_adaptive_sensitivity_level =
            to.noise_canceling_adaptive_sensitivity_level;
        sequence.push(current);
    }

    if current.manual_noise_canceling != to.manual_noise_canceling {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(current);
        }
        if current.noise_canceling_mode != a3959::structures::NoiseCancelingMode::Manual {
            current.noise_canceling_mode = a3959::structures::NoiseCancelingMode::Manual;
            sequence.push(current);
        }
        current.manual_noise_canceling = to.manual_noise_canceling;
        sequence.push(current);
    }

    if current.multi_scene_anc != to.multi_scene_anc {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(current);
        }
        if current.noise_canceling_mode != a3959::structures::NoiseCancelingMode::MultiScene {
            current.noise_canceling_mode = a3959::structures::NoiseCancelingMode::MultiScene;
            sequence.push(current);
        }
        current.multi_scene_anc = to.multi_scene_anc;
        sequence.push(current);
    }

    if current.noise_canceling_mode != to.noise_canceling_mode {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(current);
        }
        current.noise_canceling_mode = to.noise_canceling_mode;
        sequence.push(current);
    }

    if current.transparency_mode != to.transparency_mode || current.wind_noise != to.wind_noise {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::Transparency {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::Transparency;
            sequence.push(current);
        }
        current.transparency_mode = to.transparency_mode;
        current.wind_noise = to.wind_noise;
        sequence.push(current);
    }

    if current.ambient_sound_mode != to.ambient_sound_mode {
        current.ambient_sound_mode = to.ambient_sound_mode;
        sequence.push(current);
    }

    sequence
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;
    fn is_change_valid(
        from: a3959::structures::SoundModes,
        to: a3959::structures::SoundModes,
    ) -> bool {
        if from.transparency_mode != to.transparency_mode || from.wind_noise != to.wind_noise {
            if from.ambient_sound_mode != common::structures::AmbientSoundMode::Transparency
                || from.ambient_sound_mode != to.ambient_sound_mode
            {
                return false;
            }
        }

        if from.noise_canceling_mode != to.noise_canceling_mode {
            if from.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling
                || from.ambient_sound_mode != to.ambient_sound_mode
            {
                return false;
            }
        }

        if from.adaptive_noise_canceling != to.adaptive_noise_canceling
            || from.noise_canceling_adaptive_sensitivity_level
                != to.noise_canceling_adaptive_sensitivity_level
        {
            if from.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling
                || from.ambient_sound_mode != to.ambient_sound_mode
            {
                return false;
            }

            if from.noise_canceling_mode != a3959::structures::NoiseCancelingMode::Adaptive
                || from.noise_canceling_mode != to.noise_canceling_mode
            {
                return false;
            }
        }

        if from.manual_noise_canceling != to.manual_noise_canceling {
            if from.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling
                || from.ambient_sound_mode != to.ambient_sound_mode
            {
                return false;
            }

            if from.noise_canceling_mode != a3959::structures::NoiseCancelingMode::Manual
                || from.noise_canceling_mode != to.noise_canceling_mode
            {
                return false;
            }
        }

        if from.multi_scene_anc != to.multi_scene_anc {
            if from.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling
                || from.ambient_sound_mode != to.ambient_sound_mode
            {
                return false;
            }

            if from.noise_canceling_mode != a3959::structures::NoiseCancelingMode::MultiScene
                || from.noise_canceling_mode != to.noise_canceling_mode
            {
                return false;
            }
        }
        true
    }

    #[quickcheck]
    fn valid_state_transition(
        from: a3959::structures::SoundModes,
        to: a3959::structures::SoundModes,
    ) -> bool {
        let plan = create_change_plan(from, to);
        if from == to {
            return plan.is_empty();
        }
        if plan.is_empty() {
            return false;
        }
        // the initial state is not part of the plan, so check that separately
        if !is_change_valid(from, *plan.first().unwrap()) {
            return false;
        }
        // the final state is a part of the plan, though
        if *plan.last().unwrap() != to {
            return false;
        }
        // the worst case scenario should be 11 steps
        if plan.len() > 11 {
            return false;
        }

        plan.windows(2)
            .all(|change| is_change_valid(change[0], change[1]))
    }
}
