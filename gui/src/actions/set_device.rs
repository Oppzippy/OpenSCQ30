use std::rc::Rc;

use anyhow::anyhow;
use gtk::glib::{clone, MainContext};
use macaddr::MacAddr6;
use openscq30_lib::api::{
    connection::ConnectionStatus,
    device::{Device, DeviceRegistry},
};
use tokio::sync::{mpsc::UnboundedSender, oneshot, watch};

use crate::{actions, settings::SettingsFile};
use crate::{objects::GlibDevice, settings::Config};

use super::{State, StateUpdate};

pub async fn set_device<T>(
    state: &Rc<State<T>>,
    config: Rc<SettingsFile<Config>>,
    mac_address: Option<MacAddr6>,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    // Clean up any existing devices
    if let Some(handle) = &*state.connect_to_device_handle.borrow_mut() {
        handle.abort();
    }
    *state.selected_device.borrow_mut() = None;

    let Some(mac_address) = mac_address else {
        // Disconnect
        state
            .state_update_sender
            .send(StateUpdate::SetSelectedDevice(None))
            .map_err(|err| anyhow!("{err}"))?;
        state
            .state_update_sender
            .send(StateUpdate::SetLoading(false))
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        return Ok(());
    };

    // Connect to new device
    state
        .state_update_sender
        .send(StateUpdate::SetLoading(true))
        .unwrap();
    let (sender, receiver) = oneshot::channel();
    let handle = MainContext::default().spawn_local(clone!(@strong state => async move {
        let result = connect_to_device(&state, &config, mac_address).await;
        state.state_update_sender.send(StateUpdate::SetLoading(false)).unwrap();
        sender.send(result).unwrap();
    }));
    *state.connect_to_device_handle.borrow_mut() = Some(handle);
    match receiver.await {
        Ok(result) => result,
        // we don't care if the receive fails, since that just means that someone else set a new device and canceled us
        Err(_) => Ok(()),
    }
}

async fn connect_to_device<T>(
    state: &Rc<State<T>>,
    settings_file: &SettingsFile<Config>,
    mac_address: MacAddr6,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let Some(device) = state.registry.device(mac_address).await? else {
        state
            .state_update_sender
            .send(StateUpdate::SetSelectedDevice(None))
            .map_err(|err| anyhow!("{err:?}"))?;
        anyhow::bail!("device not found: {mac_address}");
    };

    *state.selected_device.borrow_mut() = Some(device.to_owned());
    let receiver = device.subscribe_to_state_updates();
    state
        .state_update_receiver
        .replace_receiver(Some(receiver))
        .await;

    let connection_status_receiver = device.connection_status();
    let state_update_sender = state.state_update_sender.clone();
    MainContext::default().spawn_local(async move {
        wait_for_disconnect(connection_status_receiver).await;
        handle_disconnect(state_update_sender);
    });

    let name = device.name().await?;
    let mac_address = device.mac_address().await?;
    let device_state = device.state().await;
    state
        .state_update_sender
        .send(StateUpdate::SetSelectedDevice(Some(GlibDevice::new(
            &name,
            &mac_address.to_string(),
        ))))
        .map_err(|err| anyhow!("{err:?}"))?;

    state
        .state_update_sender
        .send(StateUpdate::SetDeviceState(device_state))
        .map_err(|err| anyhow!("{err:?}"))?;

    actions::refresh_quick_presets(&state, &settings_file, device.service_uuid())?;

    Ok(())
}

async fn wait_for_disconnect(mut connection_status_receiver: watch::Receiver<ConnectionStatus>) {
    loop {
        match connection_status_receiver.changed().await {
            Ok(_) => {
                let connection_status = *connection_status_receiver.borrow();
                if connection_status == ConnectionStatus::Disconnected {
                    return;
                }
            }
            Err(err) => {
                tracing::debug!("connection status sender destroyed, exiting loop: {err:?}");
                break;
            }
        };
    }
}

fn handle_disconnect(state_update_sender: UnboundedSender<StateUpdate>) {
    if let Err(err) =
        state_update_sender.send(StateUpdate::AddToast("Device Disconnected".to_string()))
    {
        tracing::error!("error sending toast: {err:?}");
    }
    if let Err(err) = state_update_sender.send(StateUpdate::SetSelectedDevice(None)) {
        tracing::error!("setting device state to disconnected after receiving ConnectionStatus::Disconnected: {err:?}");
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, rc::Rc};

    use macaddr::MacAddr6;
    use mockall::predicate;
    use openscq30_lib::{
        api::connection::ConnectionStatus,
        packets::structures::{
            AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode, PresetEqualizerProfile,
            SoundModes,
        },
        state::DeviceState,
    };
    use tokio::sync::{broadcast, watch};
    use uuid::Uuid;

    use crate::{
        actions::{State, StateUpdate},
        mock::{MockDevice, MockDeviceRegistry},
        objects::GlibDevice,
        settings::SettingsFile,
    };

    use super::set_device;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let (_sender, receiver) = watch::channel(ConnectionStatus::Connected);
        let mut registry = MockDeviceRegistry::new();
        let device_state = DeviceState {
            sound_modes: Some(SoundModes {
                ambient_sound_mode: AmbientSoundMode::Transparency,
                noise_canceling_mode: NoiseCancelingMode::Indoor,
                ..Default::default()
            }),
            equalizer_configuration: EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::Acoustic,
            ),
            ..Default::default()
        };

        let device_state_2 = device_state.clone();
        registry
            .expect_device()
            .with(predicate::eq(MacAddr6::nil()))
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
                    .expect_name()
                    .once()
                    .returning(|| Ok("Test Device".into()));
                device
                    .expect_mac_address()
                    .once()
                    .returning(|| Ok(MacAddr6::nil()));
                device
                    .expect_connection_status()
                    .once()
                    .return_const(receiver);
                device.expect_service_uuid().return_const(Uuid::default());
                device.expect_state().once().return_const(device_state_2);

                Ok(Some(Rc::new(device)))
            });

        let (state, mut receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = Rc::new(SettingsFile::new(file.path().to_path_buf()));
        set_device(&state, settings_file, Some(MacAddr6::nil()))
            .await
            .unwrap();
        let mut expected_sequence = VecDeque::from([
            StateUpdate::SetLoading(true),
            StateUpdate::SetSelectedDevice(Some(GlibDevice::new(
                "Test Device",
                "00:00:00:00:00:00",
            ))),
            StateUpdate::SetDeviceState(device_state),
            StateUpdate::SetQuickPresets(Vec::new()),
            StateUpdate::SetLoading(false),
        ]);
        loop {
            if let Some(state_update) = receiver.recv().await {
                let expected = expected_sequence.pop_front().unwrap();

                match (&expected, &state_update) {
                    // the Eq implementation for DeviceObject compares identity rather than prop values,
                    // so we need custom handling
                    (
                        StateUpdate::SetSelectedDevice(expected),
                        StateUpdate::SetSelectedDevice(state_update),
                    ) => {
                        assert_eq!(
                            expected.as_ref().unwrap().name(),
                            state_update.as_ref().unwrap().name()
                        );
                        assert_eq!(
                            expected.as_ref().unwrap().mac_address(),
                            state_update.as_ref().unwrap().mac_address()
                        );
                    }
                    // Everyhing else is ok to just compare normally
                    _ => {
                        assert_eq!(expected, state_update);
                    }
                }

                if expected_sequence.is_empty() {
                    break;
                }
            }
        }
    }

    #[gtk::test]
    async fn test_sets_device_to_none_when_disconnected_is_received() {
        crate::load_resources();
        let (sender, receiver) = watch::channel(ConnectionStatus::Connected);
        let mut registry = MockDeviceRegistry::new();
        registry
            .expect_device()
            .with(predicate::eq(MacAddr6::nil()))
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
                    .expect_name()
                    .once()
                    .returning(|| Ok("Test Device".into()));
                device
                    .expect_mac_address()
                    .once()
                    .returning(|| Ok(MacAddr6::nil()));
                device
                    .expect_connection_status()
                    .once()
                    .return_const(receiver);
                device.expect_service_uuid().return_const(Uuid::default());
                device.expect_state().once().return_const(DeviceState {
                    sound_modes: Some(SoundModes {
                        ambient_sound_mode: AmbientSoundMode::Transparency,
                        noise_canceling_mode: NoiseCancelingMode::Indoor,
                        ..Default::default()
                    }),
                    equalizer_configuration: EqualizerConfiguration::new_from_preset_profile(
                        PresetEqualizerProfile::Acoustic,
                    ),
                    ..Default::default()
                });

                Ok(Some(Rc::new(device)))
            });

        let (state, _receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = Rc::new(SettingsFile::new(file.path().to_path_buf()));
        set_device(&state, settings_file, Some(MacAddr6::nil()))
            .await
            .unwrap();
        sender.send_replace(ConnectionStatus::Disconnected);
    }
}
