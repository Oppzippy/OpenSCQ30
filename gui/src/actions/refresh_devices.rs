use std::rc::Rc;

use gtk::glib::{clone, MainContext};
use openscq30_lib::api::device::{DeviceDescriptor, DeviceRegistry};

use crate::widgets::Device;

use super::{State, StateUpdate};

pub fn refresh_devices<T>(state: &Rc<State<T>>)
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    let main_context = MainContext::default();
    main_context.spawn_local(clone!(@strong state => async move {
        match state.registry.device_descriptors().await {
            Ok(descriptors) => {
                let model_devices = descriptors
                    .iter()
                    .map(|descriptor| Device {
                        mac_address: descriptor.mac_address().to_owned(),
                        name: descriptor.name().to_owned(),
                    })
                    .collect::<Vec<_>>();
                if model_devices.is_empty() {
                    state.state_update_receiver.replace_receiver(None).await;
                    *state.selected_device.borrow_mut() = None;
                }
                state.state_update_sender.send(StateUpdate::SetDevices(model_devices)).expect("error sending");
            }
            Err(err) => {
                tracing::warn!("error obtaining device descriptors: {err}")
            }
        }
    }));
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{State, StateUpdate},
        mock::{MockDescriptor, MockDeviceRegistry},
    };

    use super::refresh_devices;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let mut registry = MockDeviceRegistry::new();
        registry.expect_device_descriptors().return_once(|| {
            let mut descriptor = MockDescriptor::new();
            descriptor.expect_name().return_const("Name".to_string());
            descriptor
                .expect_mac_address()
                .return_const("MAC Address".to_string());
            Ok(vec![descriptor])
        });

        let (state, mut receiver) = State::new(registry);

        refresh_devices(&state);
        let value = receiver.recv().await.expect("should receiver state update");
        match value {
            StateUpdate::SetDevices(devices) => assert_eq!(1, devices.len()),
            _ => panic!("got wrong state update: {value:?}"),
        }
    }
}
