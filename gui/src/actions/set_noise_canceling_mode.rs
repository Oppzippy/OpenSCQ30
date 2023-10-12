use anyhow::bail;
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::{NoiseCancelingMode, SoundModes},
};

use super::State;

pub async fn set_noise_canceling_mode<T>(
    state: &State<T>,
    noise_canceling_mode: NoiseCancelingMode,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    let device_state = device.state().await;
    let Some(sound_modes) = device_state.sound_modes else {
        bail!("set_noise_canceling_mode: sound modes not supported");
    };
    let new_sound_modes = SoundModes {
        noise_canceling_mode,
        ..sound_modes
    };

    device.set_sound_modes(new_sound_modes).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use mockall::predicate;
    use openscq30_lib::{
        packets::structures::{NoiseCancelingMode, SoundModes},
        state::DeviceState,
    };

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_noise_canceling_mode;

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
                sound_modes: Some(SoundModes {
                    noise_canceling_mode: NoiseCancelingMode::Indoor,
                    ..Default::default()
                }),
                ..Default::default()
            });
        selected_device
            .expect_set_sound_modes()
            .once()
            .with(predicate::function(|sound_modes: &SoundModes| {
                sound_modes.noise_canceling_mode == NoiseCancelingMode::Transport
            }))
            .return_once(|_noise_canceling_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        set_noise_canceling_mode(&state, NoiseCancelingMode::Transport)
            .await
            .unwrap();
    }
}
