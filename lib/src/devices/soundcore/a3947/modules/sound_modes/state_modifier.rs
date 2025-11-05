use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3947,
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
    T: Has<a3947::structures::SoundModes> + Clone + Send + Sync,
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
                .send_with_response(&a3947::packets::set_sound_modes(&step))
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
    from: a3947::structures::SoundModes,
    to: a3947::structures::SoundModes,
) -> Vec<a3947::structures::SoundModes> {
    let mut sequence = Vec::new();
    let mut current = from;

    if current.ambient_sound_mode == common::structures::AmbientSoundMode::NoiseCanceling
        || to.ambient_sound_mode == common::structures::AmbientSoundMode::Transparency
    {
        set_noise_canceling_mode_dependants(&mut current, &to, &mut sequence);
        set_noise_canceling_mode(&mut current, &to, &mut sequence);
        set_transparency_mode(&mut current, &to, &mut sequence);
    } else {
        set_transparency_mode(&mut current, &to, &mut sequence);
        set_noise_canceling_mode_dependants(&mut current, &to, &mut sequence);
        set_noise_canceling_mode(&mut current, &to, &mut sequence);
    }
    set_wind_noise_reduction(&mut current, &to, &mut sequence);
    set_ambient_sound_mode(&mut current, &to, &mut sequence);

    sequence
}

fn set_ambient_sound_mode(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    if current.ambient_sound_mode != to.ambient_sound_mode {
        current.ambient_sound_mode = to.ambient_sound_mode;
        sequence.push(*current);
    }
}

fn set_transparency_mode(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    if current.transparency_mode != to.transparency_mode || current.wind_noise != to.wind_noise {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::Transparency {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::Transparency;
            sequence.push(*current);
        }
        current.transparency_mode = to.transparency_mode;
        current.wind_noise = to.wind_noise;
        sequence.push(*current);
    }
}

fn set_noise_canceling_mode(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    if current.noise_canceling_mode != to.noise_canceling_mode {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(*current);
        }
        current.noise_canceling_mode = to.noise_canceling_mode;
        sequence.push(*current);
    }
}

fn set_wind_noise_reduction(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    if current.wind_noise != to.wind_noise {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling
            && current.ambient_sound_mode != common::structures::AmbientSoundMode::Transparency
        {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(*current);
        }
        current.wind_noise = to.wind_noise;
        sequence.push(*current);
    }
}

fn set_noise_canceling_mode_dependants(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    let mut actions = [
        set_adaptive_noise_canceling,
        set_manual_noise_canceling,
        set_transportation_mode,
    ];
    // Start by changing settings dependant on the current noise canceling mode, and end by changing
    // settings dependant on the target noise canceling mode
    match (current.noise_canceling_mode, to.noise_canceling_mode) {
        (
            a3947::structures::NoiseCancelingMode::Adaptive,
            a3947::structures::NoiseCancelingMode::Manual,
        ) => actions.swap(1, 2),
        (a3947::structures::NoiseCancelingMode::Adaptive, _) => (),

        (
            a3947::structures::NoiseCancelingMode::Manual,
            a3947::structures::NoiseCancelingMode::Adaptive,
        ) => actions.rotate_left(1),
        (a3947::structures::NoiseCancelingMode::Manual, _) => actions.swap(0, 1),

        (
            a3947::structures::NoiseCancelingMode::Transportation,
            a3947::structures::NoiseCancelingMode::Manual,
        ) => actions.rotate_right(1),
        (a3947::structures::NoiseCancelingMode::Transportation, _) => actions.swap(0, 2),
    }
    actions
        .into_iter()
        .for_each(|action| action(current, to, sequence));
}

fn set_adaptive_noise_canceling(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    if current.adaptive_noise_canceling != to.adaptive_noise_canceling
        || current.environment_detection != to.environment_detection
    {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(*current);
        }
        if current.noise_canceling_mode != a3947::structures::NoiseCancelingMode::Adaptive {
            current.noise_canceling_mode = a3947::structures::NoiseCancelingMode::Adaptive;
            sequence.push(*current);
        }
        current.adaptive_noise_canceling = to.adaptive_noise_canceling;
        current.environment_detection = to.environment_detection;
        sequence.push(*current);
    }
}

fn set_manual_noise_canceling(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    if current.manual_noise_canceling != to.manual_noise_canceling {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(*current);
        }
        if current.noise_canceling_mode != a3947::structures::NoiseCancelingMode::Manual {
            current.noise_canceling_mode = a3947::structures::NoiseCancelingMode::Manual;
            sequence.push(*current);
        }
        current.manual_noise_canceling = to.manual_noise_canceling;
        sequence.push(*current);
    }
}

fn set_transportation_mode(
    current: &mut a3947::structures::SoundModes,
    to: &a3947::structures::SoundModes,
    sequence: &mut Vec<a3947::structures::SoundModes>,
) {
    if current.transportation_mode != to.transportation_mode {
        if current.ambient_sound_mode != common::structures::AmbientSoundMode::NoiseCanceling {
            current.ambient_sound_mode = common::structures::AmbientSoundMode::NoiseCanceling;
            sequence.push(*current);
        }
        if current.noise_canceling_mode != a3947::structures::NoiseCancelingMode::Transportation {
            current.noise_canceling_mode = a3947::structures::NoiseCancelingMode::Transportation;
            sequence.push(*current);
        }
        current.transportation_mode = to.transportation_mode;
        sequence.push(*current);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{self, AtomicUsize};

    use proptest::{
        prelude::*,
        proptest,
        test_runner::{Config, TestRunner},
    };

    use super::*;

    fn assert_valid_change(from: a3947::structures::SoundModes, to: a3947::structures::SoundModes) {
        if from.transparency_mode != to.transparency_mode || from.wind_noise != to.wind_noise {
            assert_eq!(
                from.ambient_sound_mode,
                common::structures::AmbientSoundMode::Transparency
            );
            assert_eq!(from.ambient_sound_mode, to.ambient_sound_mode);
        }

        if from.noise_canceling_mode != to.noise_canceling_mode {
            assert_eq!(
                from.ambient_sound_mode,
                common::structures::AmbientSoundMode::NoiseCanceling
            );
            assert_eq!(from.ambient_sound_mode, to.ambient_sound_mode);
        }

        if from.adaptive_noise_canceling != to.adaptive_noise_canceling
            || from.environment_detection != to.environment_detection
        {
            assert_eq!(
                from.ambient_sound_mode,
                common::structures::AmbientSoundMode::NoiseCanceling
            );
            assert_eq!(from.ambient_sound_mode, to.ambient_sound_mode);

            assert_eq!(
                from.noise_canceling_mode,
                a3947::structures::NoiseCancelingMode::Adaptive
            );
            assert_eq!(from.noise_canceling_mode, to.noise_canceling_mode);
        }

        if from.manual_noise_canceling != to.manual_noise_canceling {
            assert_eq!(
                from.ambient_sound_mode,
                common::structures::AmbientSoundMode::NoiseCanceling
            );
            assert_eq!(from.ambient_sound_mode, to.ambient_sound_mode);

            assert_eq!(
                from.noise_canceling_mode,
                a3947::structures::NoiseCancelingMode::Manual
            );
            assert_eq!(from.noise_canceling_mode, to.noise_canceling_mode);
        }

        if from.transportation_mode != to.transportation_mode {
            assert_eq!(
                from.ambient_sound_mode,
                common::structures::AmbientSoundMode::NoiseCanceling
            );
            assert_eq!(from.ambient_sound_mode, to.ambient_sound_mode);

            assert_eq!(
                from.noise_canceling_mode,
                a3947::structures::NoiseCancelingMode::Transportation
            );
            assert_eq!(from.noise_canceling_mode, to.noise_canceling_mode);
        }

        if from.wind_noise != to.wind_noise {
            assert!(
                from.ambient_sound_mode == common::structures::AmbientSoundMode::NoiseCanceling
                    || from.ambient_sound_mode
                        == common::structures::AmbientSoundMode::Transparency,
                "{:?}",
                from.ambient_sound_mode
            );
        }
    }

    proptest! {
        #[test]
        fn test_valid_state_transitions(
            from: a3947::structures::SoundModes,
            to: a3947::structures::SoundModes,
        ) {
            let plan = create_change_plan(from, to);

            assert_eq!(from == to, plan.is_empty());

            if !plan.is_empty() {
                // the initial state is not part of the plan, so check that separately
                assert_valid_change(from, *plan.first().unwrap());
                // the final state is a part of the plan, though
                assert_eq!(*plan.last().unwrap(), to);

                plan.windows(2)
                    .for_each(|change| assert_valid_change(change[0], change[1]))
            }
        }

        #[test]
        fn test_worst_case(
            from: a3947::structures::SoundModes,
            to: a3947::structures::SoundModes,
        ) {
            let plan = create_change_plan(from, to);

            assert!(plan.len() <= 10, "{} steps", plan.len());
        }
    }

    #[test]
    fn average_case() {
        let mut runner = TestRunner::new(Config {
            cases: 10000,
            rng_algorithm: prop::test_runner::RngAlgorithm::ChaCha,
            rng_seed: prop::test_runner::RngSeed::Fixed(0),
            ..Config::default()
        });
        let total = AtomicUsize::new(0);
        runner
            .run(
                &(
                    a3947::structures::SoundModes::arbitrary(),
                    a3947::structures::SoundModes::arbitrary(),
                ),
                |(from, to)| {
                    let steps = create_change_plan(from, to).len();
                    total.fetch_add(steps, atomic::Ordering::Relaxed);
                    Ok(())
                },
            )
            .unwrap();
        let total = total.load(atomic::Ordering::Relaxed);
        let average = total as f64 / runner.config().cases as f64;

        // round up to nearest 10th for leeway
        assert!(average <= 7.4, "average case: {average} steps",);
    }
}
