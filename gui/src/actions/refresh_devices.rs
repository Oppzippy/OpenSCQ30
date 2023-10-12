use openscq30_lib::api::device::{DeviceDescriptor, DeviceRegistry};

use crate::objects::GlibDevice;

use super::{State, StateUpdate};

pub async fn refresh_devices<T>(state: &State<T>) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    if !state.is_refresh_in_progress.get() {
        let descriptors = {
            state.is_refresh_in_progress.set(true);
            let descriptors_result = state.registry.device_descriptors().await;
            state.is_refresh_in_progress.set(false);
            descriptors_result?
        };

        let model_devices = descriptors
            .iter()
            .map(|descriptor| {
                GlibDevice::new(descriptor.name(), &descriptor.mac_address().to_string())
            })
            .collect::<Vec<_>>();

        if model_devices.is_empty() {
            // Selection will not change automatically if device list is empty
            state.state_update_receiver.replace_receiver(None).await;
            *state.selected_device.borrow_mut() = None;
        }
        state
            .state_update_sender
            .send(StateUpdate::SetDevices(model_devices))
            .map_err(|err| anyhow::anyhow!("{err}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::refresh_devices;
    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
    };
    use gtk::glib::{self, clone, MainContext};
    use macaddr::MacAddr6;
    use openscq30_lib::api::device::GenericDeviceDescriptor;
    use std::time::Duration;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let mut registry = MockDeviceRegistry::new();
        registry.expect_device_descriptors().return_once(|| {
            let descriptor = GenericDeviceDescriptor::new("Test Device", MacAddr6::nil());
            Ok(vec![descriptor])
        });

        let (state, mut receiver) = State::new(registry);

        refresh_devices(&state).await.unwrap();
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
            let descriptor = GenericDeviceDescriptor::new("Test Device", MacAddr6::nil());
            Ok(vec![descriptor])
        });

        let (state, mut receiver) = State::new(registry);

        MainContext::default().spawn_local(clone!(@strong state => async move {
            refresh_devices(&state).await.unwrap();
        }));
        MainContext::default().spawn_local(clone!(@strong state => async move {
            refresh_devices(&state).await.unwrap();
        }));
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
        refresh_devices(&state).await.unwrap();
        let _third_state_update = receiver
            .recv()
            .await
            .expect("should receive third state update");
    }
}
