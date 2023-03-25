use std::rc::Rc;

use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::AmbientSoundMode,
};

use super::State;

pub async fn set_ambient_sound_mode<T>(
    state: &Rc<State<T>>,
    ambient_sound_mode_id: u8,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    let ambient_sound_mode = AmbientSoundMode::from_id(ambient_sound_mode_id)
        .ok_or_else(|| anyhow::anyhow!("invalid ambient sound mode: {ambient_sound_mode_id}"))?;
    device.set_ambient_sound_mode(ambient_sound_mode).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;
    use openscq30_lib::packets::structures::AmbientSoundMode;

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_ambient_sound_mode;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);
        let mut selected_device = MockDevice::new();
        selected_device
            .expect_set_ambient_sound_mode()
            .once()
            .with(predicate::eq(AmbientSoundMode::NoiseCanceling))
            .return_once(|_ambient_sound_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Arc::new(selected_device));

        set_ambient_sound_mode(&state, AmbientSoundMode::NoiseCanceling.id())
            .await
            .unwrap();
    }
}
