use std::rc::Rc;

use gtk::glib::{clone, MainContext};
use openscq30_lib::api::device::{DeviceDescriptor, DeviceRegistry};

use crate::widgets::Device;

use super::{State, StateUpdate};

pub fn refresh_devices<T>(state: &Rc<State<T>>)
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    if !state.is_refresh_in_progress.get() {
        state.is_refresh_in_progress.set(true);
        MainContext::default().spawn_local(clone!(@strong state => async move {
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
            state.is_refresh_in_progress.set(false);
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::refresh_devices;
    use crate::{
        actions::{State, StateUpdate},
        mock::{MockDescriptor, MockDeviceRegistry},
    };
    use gtk::glib;
    use std::time::Duration;

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
        let value = receiver.recv().await.expect("should receive state update");
        match value {
            StateUpdate::SetDevices(devices) => assert_eq!(1, devices.len()),
            _ => panic!("got wrong state update: {value:?}"),
        }
    }

    #[gtk::test]
    async fn it_doesnt_run_multiple_times_concurrently() {
        crate::load_resources();
        let mut registry = MockDeviceRegistry::new();
        registry.expect_device_descriptors().times(2).returning(|| {
            let mut descriptor = MockDescriptor::new();
            descriptor.expect_name().return_const("Name".to_string());
            descriptor
                .expect_mac_address()
                .return_const("MAC Address".to_string());
            Ok(vec![descriptor])
        });

        let (state, mut receiver) = State::new(registry);

        refresh_devices(&state);
        refresh_devices(&state);
        let _first_state_update = receiver
            .recv()
            .await
            .expect("should receive first state update");
        let second_state_update = tokio::select! {
            second_state_update = receiver.recv() => second_state_update,
            _timeout = glib::timeout_future(Duration::from_millis(50)) => None,
        };
        assert_eq!(
            None, second_state_update,
            "should not receive second state update"
        );
        refresh_devices(&state);
        let _third_state_update = receiver
            .recv()
            .await
            .expect("should receive third state update");
    }
}
