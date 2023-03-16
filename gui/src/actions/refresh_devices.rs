use std::{cell::RefCell, rc::Rc, sync::Arc};

use gtk::glib::{clone, MainContext};
use openscq30_lib::{
    api::device::{DeviceDescriptor, DeviceRegistry},
    state::DeviceState,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{swappable_broadcast::SwappableBroadcastReceiver, widgets::Device};

use super::StateUpdate;

pub fn refresh_devices<T>(
    state_updates: &UnboundedSender<StateUpdate>,
    registry: &Arc<T>,
    selected_device: &Rc<RefCell<Option<Arc<T::DeviceType>>>>,
    state_update_receiver: &Rc<SwappableBroadcastReceiver<DeviceState>>,
) where
    T: DeviceRegistry + Send + Sync + 'static,
{
    let main_context = MainContext::default();
    main_context.spawn_local(clone!(@strong state_updates, @strong registry, @strong selected_device, @strong state_update_receiver => async move {
        match registry.device_descriptors().await {
            Ok(descriptors) => {
                let model_devices = descriptors
                    .iter()
                    .map(|descriptor| Device {
                        mac_address: descriptor.mac_address().to_owned(),
                        name: descriptor.name().to_owned(),
                    })
                    .collect::<Vec<_>>();
                if model_devices.is_empty() {
                    state_update_receiver.replace_receiver(None).await;
                    *selected_device.borrow_mut() = None;
                }
                state_updates.send(StateUpdate::SetDevices(model_devices)).expect("error sending");
            }
            Err(err) => {
                tracing::warn!("error obtaining device descriptors: {err}")
            }
        }
    }));
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, sync::Arc};

    use tokio::sync::mpsc;

    use crate::{
        actions::StateUpdate,
        mock::{MockDescriptor, MockDeviceRegistry},
        swappable_broadcast::SwappableBroadcastReceiver,
    };

    use super::refresh_devices;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let swappable_receiver = SwappableBroadcastReceiver::new();
        let mut registry = MockDeviceRegistry::new();
        let selected_device = Rc::new(RefCell::new(None));

        registry.expect_device_descriptors().return_once(|| {
            let mut descriptor = MockDescriptor::new();
            descriptor.expect_name().return_const("Name".to_string());
            descriptor
                .expect_mac_address()
                .return_const("MAC Address".to_string());
            Ok(vec![descriptor])
        });

        refresh_devices(
            &sender,
            &Arc::new(registry),
            &selected_device,
            &Rc::new(swappable_receiver),
        );
        let value = receiver.recv().await.expect("should receiver state update");
        match value {
            StateUpdate::SetDevices(devices) => assert_eq!(1, devices.len()),
            // _ => panic!("got wrong state update: {value:?}"),
        }
    }
}
