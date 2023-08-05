use std::rc::Rc;

use anyhow::bail;
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::{SoundModes, TransparencyMode},
};

use super::State;

pub async fn set_transparency_mode<T>(
    state: &Rc<State<T>>,
    transparency_mode: TransparencyMode,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    let device_state = device.state().await;
    let Some(sound_modes) = device_state.sound_modes else {
        bail!("set_transparency_mode: sound modes not supported");
    };
    let new_sound_modes = SoundModes {
        transparency_mode,
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
        packets::structures::{SoundModes, TransparencyMode},
        state::DeviceState,
    };

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_transparency_mode;

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
                    transparency_mode: TransparencyMode::FullyTransparent,
                    ..Default::default()
                }),
                ..Default::default()
            });
        selected_device
            .expect_set_sound_modes()
            .once()
            .with(predicate::function(|sound_modes: &SoundModes| {
                sound_modes.transparency_mode == TransparencyMode::VocalMode
            }))
            .return_once(|_transparency_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        set_transparency_mode(&state, TransparencyMode::VocalMode)
            .await
            .unwrap();
    }
}
