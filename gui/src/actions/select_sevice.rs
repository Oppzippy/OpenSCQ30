use std::rc::Rc;

use gtk::glib::{clone, MainContext};
use openscq30_lib::api::device::{Device, DeviceRegistry};

use crate::objects::DeviceObject;

use super::{State, StateUpdate};

pub fn select_device<T>(state: &Rc<State<T>>, new_selected_device: Option<DeviceObject>)
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    // Clean up any existing devices
    if let Some(handle) = &*state.connect_to_device_handle.borrow_mut() {
        handle.abort();
    }
    *state.selected_device.borrow_mut() = None;
    state
        .state_update_sender
        .send(StateUpdate::SetLoading(false))
        .unwrap();

    // Connect to new device
    if let Some(new_selected_device) = new_selected_device {
        state
            .state_update_sender
            .send(StateUpdate::SetLoading(true))
            .unwrap();
        let main_context = MainContext::default();
        *state.connect_to_device_handle.borrow_mut() = Some(
                    main_context.spawn_local(
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
                    )
                );
    }
}
