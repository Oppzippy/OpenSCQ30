use std::rc::Rc;

use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::NoiseCancelingMode,
};

use super::State;

pub async fn set_noise_canceling_mode<T>(
    state: &Rc<State<T>>,
    noise_canceling_mode_id: u8,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;
    let noise_canceling_mode =
        NoiseCancelingMode::from_id(noise_canceling_mode_id).ok_or_else(|| {
            anyhow::anyhow!("invalid noise canceling mode: {noise_canceling_mode_id}")
        })?;
    device
        .set_noise_canceling_mode(noise_canceling_mode)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;
    use openscq30_lib::packets::structures::NoiseCancelingMode;

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
            .expect_set_noise_canceling_mode()
            .once()
            .with(predicate::eq(NoiseCancelingMode::Transport))
            .return_once(|_noise_canceling_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Arc::new(selected_device));

        set_noise_canceling_mode(&state, NoiseCancelingMode::Transport.id())
            .await
            .unwrap();
    }
}
