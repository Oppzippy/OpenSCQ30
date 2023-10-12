use std::time::Duration;

use gtk::glib::{self, clone, timeout_future, MainContext};
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::{CustomNoiseCanceling, SoundModes},
};
use tokio::sync::oneshot;

use super::State;

pub async fn set_custom_noise_canceling<T>(
    state: &State<T>,
    custom_noise_canceling: CustomNoiseCanceling,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    if let Some(handle) = state.set_custom_noise_canceling_handle.take() {
        handle.abort();
    }
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    let (result_sender, result_receiver) = oneshot::channel::<anyhow::Result<()>>();
    let new_handle = MainContext::default().spawn_local(clone!(@weak device => async move {
        timeout_future(Duration::from_millis(500)).await;
        let Some(sound_modes) = device.state().await.sound_modes else {
            result_sender.send(Err(anyhow::anyhow!("sound modes not supported"))).expect("receiver dropped");
            return;
        };

        let new_sound_modes = SoundModes {
            custom_noise_canceling,
            ..sound_modes
        };

        let result = device.set_sound_modes(new_sound_modes).await;
        result_sender.send(result.map_err(Into::into)).expect("receiver dropped");
    }));
    if let Some(old_handle) = state
        .set_equalizer_configuration_handle
        .replace(Some(new_handle))
    {
        old_handle.abort();
    }

    match result_receiver.await {
        Ok(Ok(())) => {
            tracing::trace!("returning with no error");
            Ok(())
        }
        Err(_sender_dropped) => {
            tracing::trace!("sender dropped, returning");
            Ok(())
        }
        Ok(Err(err)) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use mockall::predicate;
    use openscq30_lib::{
        packets::structures::{CustomNoiseCanceling, SoundModes},
        state::DeviceState,
    };

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_custom_noise_canceling;

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
                    custom_noise_canceling: CustomNoiseCanceling::new(1),
                    ..Default::default()
                }),
                ..Default::default()
            });
        selected_device
            .expect_set_sound_modes()
            .once()
            .with(predicate::function(|sound_modes: &SoundModes| {
                sound_modes.custom_noise_canceling == CustomNoiseCanceling::new(2)
            }))
            .return_once(|_custom_noise_canceling| Ok(()));
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        set_custom_noise_canceling(&state, CustomNoiseCanceling::new(2))
            .await
            .unwrap();
    }
}
