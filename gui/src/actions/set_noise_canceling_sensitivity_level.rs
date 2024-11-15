use std::time::Duration;

use anyhow::Context;
use gtk::glib::{self, clone, timeout_future, MainContext};
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    devices::standard::structures::SoundModesTypeTwo,
};
use tokio::sync::oneshot;

use super::State;

pub async fn set_noise_canceling_sensitivity_level<T>(
    state: &State<T>,
    sensitivity_level: u8,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    // Debounce
    let (result_sender, result_receiver) = oneshot::channel::<anyhow::Result<()>>();
    let new_handle = MainContext::default().spawn_local(clone!(
        #[weak]
        device,
        async move {
            let result = set_noise_canceling_sensitivity_level_after_delay(
                device.as_ref(),
                sensitivity_level,
            )
            .await;
            result_sender.send(result).expect("receiver dropped");
        }
    ));

    // Store the handle so we can cancel it later
    if let Some(old_handle) = state
        .set_noise_canceling_sensitivity_level_handle
        .replace(Some(new_handle))
    {
        old_handle.abort();
    }

    match result_receiver.await {
        Ok(Ok(())) => {
            tracing::trace!("set_noise_canceling_sensitivity_level: returning with no error");
            Ok(())
        }
        Err(_sender_dropped) => {
            tracing::trace!("set_noise_canceling_sensitivity_level: sender dropped, returning");
            Ok(())
        }
        Ok(Err(err)) => Err(err),
    }
}

async fn set_noise_canceling_sensitivity_level_after_delay(
    device: &impl Device,
    sensitivity_level: u8,
) -> anyhow::Result<()> {
    timeout_future(Duration::from_millis(500)).await;

    let Some(sound_modes) = device.state().await.sound_modes_type_two else {
        anyhow::bail!("sound modes type two not supported");
    };

    let new_sound_modes = SoundModesTypeTwo {
        noise_canceling_adaptive_sensitivity_level: sensitivity_level,
        ..sound_modes
    };

    device
        .set_sound_modes_type_two(new_sound_modes)
        .await
        .context("set sound modes")
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use mockall::predicate;
    use openscq30_lib::devices::standard::{state::DeviceState, structures::SoundModesTypeTwo};

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_noise_canceling_sensitivity_level;

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
                    noise_canceling_adaptive_sensitivity_level: 1,
                    ..Default::default()
                }),
                ..Default::default()
            });
        selected_device
            .expect_set_sound_modes_type_two()
            .once()
            .with(predicate::function(|sound_modes: &SoundModesTypeTwo| {
                sound_modes.noise_canceling_adaptive_sensitivity_level == 2
            }))
            .return_once(|_noise_canceling_sensitivity_level| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        set_noise_canceling_sensitivity_level(&state, 2)
            .await
            .unwrap();
    }
}
