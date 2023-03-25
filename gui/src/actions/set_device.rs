use std::rc::Rc;

use gtk::glib::{clone, MainContext};
use openscq30_lib::api::device::{Device, DeviceRegistry};

use crate::objects::DeviceObject;

use super::{State, StateUpdate};

pub fn set_device<T>(
    state: &Rc<State<T>>,
    new_selected_device: Option<DeviceObject>,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    // Clean up any existing devices
    if let Some(handle) = &*state.connect_to_device_handle.borrow_mut() {
        handle.abort();
    }
    *state.selected_device.borrow_mut() = None;

    // Connect to new device
    if let Some(new_selected_device) = new_selected_device {
        state
            .state_update_sender
            .send(StateUpdate::SetLoading(true))
            .unwrap();
        let main_context = MainContext::default();
        let handle = main_context.spawn_local(
            clone!(@strong state => async move {
                match state.registry.device(&new_selected_device.mac_address()).await {
                    Ok(Some(device)) => {
                        *state.selected_device.borrow_mut() = Some(device.to_owned());
                        let receiver = device.subscribe_to_state_updates();
                        state.state_update_receiver.replace_receiver(Some(receiver)).await;

                        let ambient_sound_mode = device.ambient_sound_mode().await;
                        let noise_canceling_mode = device.noise_canceling_mode().await;
                        let equalizer_configuration = device.equalizer_configuration().await;

                        state.state_update_sender.send(StateUpdate::SetAmbientSoundMode(ambient_sound_mode)).unwrap();
                        state.state_update_sender.send(StateUpdate::SetNoiseCancelingMode(noise_canceling_mode)).unwrap();
                        state.state_update_sender.send(StateUpdate::SetEqualizerConfiguration(equalizer_configuration)).unwrap();
                    },
                    Ok(None) => {
                        tracing::warn!("could not find selected device: {:?}", new_selected_device);
                    },
                    Err(err) => {
                        tracing::warn!("error connecting to device {:?}: {err}", new_selected_device);
                    },
                }

                state.state_update_sender.send(StateUpdate::SetLoading(false)).unwrap();
                if state.selected_device.borrow().is_none() {
                    state.state_update_sender.send(StateUpdate::SetSelectedDevice(None)).unwrap();
                }
            })
        );
        *state.connect_to_device_handle.borrow_mut() = Some(handle);
    } else {
        state
            .state_update_sender
            .send(StateUpdate::SetLoading(false))
            .map_err(|err| anyhow::anyhow!("{err}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, sync::Arc};

    use mockall::predicate;
    use openscq30_lib::packets::structures::{
        AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode, PresetEqualizerProfile,
    };
    use tokio::sync::broadcast;

    use crate::{
        actions::{State, StateUpdate},
        mock::{MockDevice, MockDeviceRegistry},
        objects::DeviceObject,
    };

    use super::set_device;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let mut registry = MockDeviceRegistry::new();
        registry
            .expect_device()
            .with(predicate::eq("00:00:00:00:00:00"))
            .return_once(|_mac_address| {
                let mut device = MockDevice::new();
                device
                    .expect_subscribe_to_state_updates()
                    .once()
                    .return_once(|| {
                        let (_sender, receiver) = broadcast::channel(10);
                        receiver
                    });
                device
                    .expect_ambient_sound_mode()
                    .once()
                    .return_const(AmbientSoundMode::Transparency);
                device
                    .expect_noise_canceling_mode()
                    .once()
                    .return_const(NoiseCancelingMode::Indoor);
                device.expect_equalizer_configuration().once().return_const(
                    EqualizerConfiguration::new_from_preset_profile(
                        PresetEqualizerProfile::Acoustic,
                    ),
                );

                Ok(Some(Arc::new(device)))
            });

        let (state, mut receiver) = State::new(registry);

        let new_selected_device =
            DeviceObject::new(&"Name".to_string(), &"00:00:00:00:00:00".to_string());
        set_device(&state, Some(new_selected_device)).unwrap();
        let mut expected_sequence = VecDeque::from([
            StateUpdate::SetLoading(true),
            StateUpdate::SetAmbientSoundMode(AmbientSoundMode::Transparency),
            StateUpdate::SetNoiseCancelingMode(NoiseCancelingMode::Indoor),
            StateUpdate::SetEqualizerConfiguration(
                EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
            ),
            StateUpdate::SetLoading(false),
        ]);
        loop {
            if let Some(state_update) = receiver.recv().await {
                let expected = expected_sequence.pop_front().unwrap();
                assert_eq!(expected, state_update);

                if expected_sequence.is_empty() {
                    break;
                }
            }
        }
    }
}
