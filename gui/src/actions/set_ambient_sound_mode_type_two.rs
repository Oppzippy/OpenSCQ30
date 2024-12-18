use anyhow::bail;
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    devices::standard::structures::{AmbientSoundMode, SoundModesTypeTwo},
};

use super::State;

pub async fn set_ambient_sound_mode_type_two<T>(
    state: &State<T>,
    ambient_sound_mode: AmbientSoundMode,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    let device_state = device.state().await;
    let Some(sound_modes) = device_state.sound_modes_type_two else {
        bail!("set_ambient_sound_mode_type_two: sound modes type two not supported");
    };
    let new_sound_modes = SoundModesTypeTwo {
        ambient_sound_mode,
        ..sound_modes
    };

    device.set_sound_modes_type_two(new_sound_modes).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use mockall::predicate;
    use openscq30_lib::devices::standard::{state::DeviceState, structures::AmbientSoundMode};

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::*;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);
        let mut selected_device = MockDevice::new();
        selected_device
            .expect_state()
            .once()
            .return_const(DeviceState {
                sound_modes_type_two: Some(SoundModesTypeTwo {
                    ambient_sound_mode: AmbientSoundMode::Normal,
                    ..Default::default()
                }),
                ..Default::default()
            });
        selected_device
            .expect_set_sound_modes_type_two()
            .once()
            .with(predicate::function(|sound_modes: &SoundModesTypeTwo| {
                sound_modes.ambient_sound_mode == AmbientSoundMode::NoiseCanceling
            }))
            .return_once(|_ambient_sound_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        set_ambient_sound_mode_type_two(&state, AmbientSoundMode::NoiseCanceling)
            .await
            .unwrap();
    }
}
