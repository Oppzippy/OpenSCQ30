use std::{cell::RefCell, rc::Rc, sync::Arc};

use gtk::glib::{clone, JoinHandle, MainContext};
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    state::DeviceState,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{objects::DeviceObject, swappable_broadcast::SwappableBroadcastReceiver};

use super::StateUpdate;

pub fn select_device<T>(
    state_updates: &UnboundedSender<StateUpdate>,
    registry: &Arc<T>,
    selected_device: &Rc<RefCell<Option<Arc<T::DeviceType>>>>,
    connect_to_device_handle: &Arc<RefCell<Option<JoinHandle<()>>>>,
    state_update_receiver: &Rc<SwappableBroadcastReceiver<DeviceState>>,
    new_selected_device: Option<DeviceObject>,
) where
    T: DeviceRegistry + Send + Sync + 'static,
{
    // Clean up any existing devices
    if let Some(handle) = &*connect_to_device_handle.borrow_mut() {
        handle.abort();
    }
    *selected_device.borrow_mut() = None;
    state_updates.send(StateUpdate::SetLoading(false)).unwrap();

    // Connect to new device
    if let Some(new_selected_device) = new_selected_device {
        state_updates.send(StateUpdate::SetLoading(true)).unwrap();
        let main_context = MainContext::default();
        *connect_to_device_handle.borrow_mut() = Some(
                    main_context.spawn_local(
                        clone!(@strong state_updates, @strong registry, @strong selected_device, @strong state_update_receiver => async move {
                            match registry.device(&new_selected_device.mac_address()).await {
                                Ok(Some(device)) => {
                                    *selected_device.borrow_mut() = Some(device.to_owned());
                                    let receiver = device.subscribe_to_state_updates();
                                    state_update_receiver.replace_receiver(Some(receiver)).await;

                                    let ambient_sound_mode = device.ambient_sound_mode().await;
                                    let noise_canceling_mode = device.noise_canceling_mode().await;
                                    let equalizer_configuration = device.equalizer_configuration().await;

                                    state_updates.send(StateUpdate::SetAmbientSoundMode(ambient_sound_mode)).unwrap();
                                    state_updates.send(StateUpdate::SetNoiseCancelingMode(noise_canceling_mode)).unwrap();
                                    state_updates.send(StateUpdate::SetEqualizerConfiguration(equalizer_configuration)).unwrap();
                                },
                                Ok(None) => {
                                    tracing::warn!("could not find selected device: {:?}", new_selected_device);
                                },
                                Err(err) => {
                                    tracing::warn!("error connecting to device {:?}: {err}", new_selected_device);
                                },
                            }

                            state_updates.send(StateUpdate::SetLoading(false)).unwrap();
                            if selected_device.borrow().is_none() {
                                state_updates.send(StateUpdate::SetSelectedDevice(None)).unwrap();
                            }
                        })
                    )
                );
    }
}
