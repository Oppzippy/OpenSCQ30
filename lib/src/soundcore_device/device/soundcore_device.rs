use std::{collections::HashMap, mem, sync::Arc};

use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch, Mutex};
use tracing::{trace, warn};
use uuid::Uuid;

use crate::{
    api::{
        self,
        connection::{Connection, ConnectionStatus},
    },
    devices::standard::{
        packets::{
            inbound::{state_update_packet::StateUpdatePacket, TryIntoInboundPacket},
            outbound::{RequestFirmwareVersionPacket, RequestStatePacket},
        },
        state::DeviceState,
        structures::{
            AmbientSoundModeCycle, CustomButtonModel, EqualizerConfiguration, HearId, SoundModes,
            SoundModesTypeTwo,
        },
    },
    futures::{Futures, JoinHandle},
};

use super::{
    device_implementation::DeviceImplementation, packet_io_controller::PacketIOController,
    soundcore_command::CommandResponse, Packet,
};

pub struct SoundcoreDevice<ConnectionType, FuturesType>
where
    ConnectionType: Connection,
    FuturesType: Futures,
{
    controller: PacketIOController<ConnectionType, FuturesType>,
    connection: Arc<ConnectionType>,
    state_sender: Arc<Mutex<watch::Sender<DeviceState>>>,
    join_handle: FuturesType::JoinHandleType,
    implementation: Arc<dyn DeviceImplementation + Send + Sync>,
}

impl<ConnectionType, FuturesType> SoundcoreDevice<ConnectionType, FuturesType>
where
    ConnectionType: Connection,
    FuturesType: Futures,
{
    pub async fn new(connection: Arc<ConnectionType>) -> crate::Result<Self> {
        let (controller, receiver) = PacketIOController::new(connection.clone()).await?;
        let (initial_state, implementation) = Self::fetch_initial_state(&controller).await?;

        let is_serial_number_missing = initial_state.serial_number.is_none();
        let packet_handlers = implementation.packet_handlers();

        let (state_sender, _) = watch::channel(initial_state);
        let state_sender = Arc::new(Mutex::new(state_sender));

        let join_handle =
            Self::spawn_inbound_packet_handler(packet_handlers, receiver, state_sender.to_owned());

        if is_serial_number_missing {
            tracing::debug!(
                "requesting serial number since it was not included in the state update packet"
            );
            // The implementation will handle the response, so we can send the request and forget about it
            controller
                .send(&RequestFirmwareVersionPacket::new().into())
                .await?;
        }

        Ok(Self {
            controller,
            connection,
            join_handle,
            state_sender,
            implementation,
        })
    }

    pub async fn fetch_initial_state(
        controller: &PacketIOController<ConnectionType, FuturesType>,
    ) -> crate::Result<(DeviceState, Arc<dyn DeviceImplementation + Send + Sync>)> {
        tracing::debug!("requesting state to determine model");
        let unparsed = controller.send(&RequestStatePacket::new().into()).await?;
        let parsed: StateUpdatePacket = unparsed.try_into_inbound_packet()?;
        let implementation = (parsed.device_profile.implementation)();
        let state = implementation.initialize(&unparsed.body)?;

        Ok((state, implementation))
    }

    fn spawn_inbound_packet_handler(
        packet_handlers: HashMap<
            [u8; 7],
            Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>,
        >,
        mut inbound_receiver: mpsc::Receiver<Packet>,
        state_sender_lock: Arc<Mutex<watch::Sender<DeviceState>>>,
    ) -> FuturesType::JoinHandleType {
        FuturesType::spawn(async move {
            while let Some(packet) = inbound_receiver.recv().await {
                match packet_handlers.get(&packet.command()) {
                    Some(handler) => {
                        let state_sender = state_sender_lock.lock().await;
                        let state = state_sender.borrow();
                        let new_state = handler(&packet.body, state.to_owned());
                        if new_state != *state {
                            trace!(event = "state_update", old_state = ?state, new_state = ?new_state);
                            mem::drop(state);
                            state_sender.send_replace(new_state);
                        }
                    }
                    None => {
                        warn!("no packet handler found for packet: {packet:?}")
                    }
                }
            }
        })
    }

    async fn handle_response(
        &self,
        response: CommandResponse,
        state_sender: &watch::Sender<DeviceState>,
    ) -> crate::Result<()> {
        self.send_packets(&response.packets).await?;
        state_sender.send_replace(response.new_state);
        Ok(())
    }

    async fn send_packets(&self, packets: &[Packet]) -> crate::Result<()> {
        for packet in packets {
            self.controller.send(packet).await?;
        }
        Ok(())
    }
}

impl<ConnectionType, FuturesType> api::device::Device
    for SoundcoreDevice<ConnectionType, FuturesType>
where
    ConnectionType: Connection,
    FuturesType: Futures,
{
    async fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState> {
        self.state_sender.lock().await.subscribe()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        self.connection.mac_address().await
    }

    async fn name(&self) -> crate::Result<String> {
        self.connection.name().await
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection.connection_status()
    }

    fn service_uuid(&self) -> Uuid {
        self.connection.service_uuid()
    }

    async fn state(&self) -> DeviceState {
        self.state_sender.lock().await.borrow().to_owned()
    }

    async fn set_sound_modes(&self, sound_modes: SoundModes) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        if state.device_features.sound_mode.is_none() {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "sound modes",
            });
        }
        let Some(prev_sound_modes) = state.sound_modes else {
            return Err(crate::Error::MissingData {
                name: "sound modes",
            });
        };
        if prev_sound_modes == sound_modes {
            return Ok(());
        }

        let response = self.implementation.set_sound_modes(state, sound_modes)?;
        self.handle_response(response, &state_sender).await?;
        Ok(())
    }

    async fn set_sound_modes_type_two(&self, sound_modes: SoundModesTypeTwo) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        let Some(prev_sound_modes) = state.sound_modes_type_two else {
            return Err(crate::Error::MissingData {
                name: "sound modes type two",
            });
        };
        if prev_sound_modes == sound_modes {
            return Ok(());
        }

        let response = self
            .implementation
            .set_sound_modes_type_two(state, sound_modes)?;
        self.handle_response(response, &state_sender).await?;
        Ok(())
    }

    async fn set_ambient_sound_mode_cycle(
        &self,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        if !state.device_features.has_ambient_sound_mode_cycle {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "ambient sound mode cycle",
            });
        }
        let Some(prev_cycle) = state.ambient_sound_mode_cycle else {
            return Err(crate::Error::MissingData {
                name: "ambient sound mode cycle",
            });
        };
        if prev_cycle == cycle {
            return Ok(());
        }

        let response = self
            .implementation
            .set_ambient_sound_mode_cycle(state, cycle)?;
        self.handle_response(response, &state_sender).await?;
        Ok(())
    }

    async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();

        if state.device_features.num_equalizer_channels == 0 {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "equalizer",
            });
        }
        if equalizer_configuration
            .volume_adjustments()
            .adjustments()
            .len()
            != state.device_features.num_equalizer_bands
        {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "wrong number of equalizer bands",
            });
        }
        if equalizer_configuration == state.equalizer_configuration {
            return Ok(());
        }

        let response = self
            .implementation
            .set_equalizer_configuration(state, equalizer_configuration)?;
        self.handle_response(response, &state_sender).await?;
        Ok(())
    }

    async fn set_hear_id(&self, hear_id: HearId) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();

        if !state.device_features.has_hear_id {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "hear id",
            });
        }

        let response = self.implementation.set_hear_id(state, hear_id)?;
        self.handle_response(response, &state_sender).await?;
        Ok(())
    }

    async fn set_custom_button_model(
        &self,
        custom_button_model: CustomButtonModel,
    ) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();

        if !state.device_features.has_custom_button_model {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "custom button model",
            });
        }

        let prev_custom_button_model =
            state.custom_button_model.ok_or(crate::Error::MissingData {
                name: "custom button model",
            })?;
        if custom_button_model == prev_custom_button_model {
            return Ok(());
        }

        let response = self
            .implementation
            .set_custom_button_model(state, custom_button_model)?;
        self.handle_response(response, &state_sender).await?;
        Ok(())
    }
}

impl<ConnectionType, FuturesType> Drop for SoundcoreDevice<ConnectionType, FuturesType>
where
    ConnectionType: Connection,
    FuturesType: Futures,
{
    fn drop(&mut self) {
        self.join_handle.abort();
    }
}

impl<ConnectionType, FuturesType> std::fmt::Debug for SoundcoreDevice<ConnectionType, FuturesType>
where
    ConnectionType: Connection,
    FuturesType: Futures,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoundcoreDevice").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use macaddr::MacAddr6;
    use tokio::sync::{mpsc, Mutex};

    use super::SoundcoreDevice;
    use crate::{
        api::device::Device,
        devices::standard::{
            packets::inbound::{
                FirmwareVersionUpdatePacket, InboundPacket, SetEqualizerOkPacket,
                SetSoundModeOkPacket,
            },
            structures::{
                AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
                SoundModes, VolumeAdjustments,
            },
        },
        futures::TokioFutures,
        soundcore_device::device::Packet,
        stub::connection::StubConnection,
    };

    fn example_state_update_packet() -> Vec<u8> {
        vec![
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ]
    }

    fn example_firmware_version_packet() -> Vec<u8> {
        Packet {
            command: FirmwareVersionUpdatePacket::header(),
            body: "02.0002.000000000000003028".as_bytes().to_vec(),
        }
        .bytes()
    }

    async fn create_test_connection() -> (Arc<StubConnection>, mpsc::Sender<Vec<u8>>) {
        let connection = Arc::new(StubConnection::new());
        connection
            .set_name_return(Ok("Soundcore Q30".to_string()))
            .await;
        connection.set_mac_address_return(Ok(MacAddr6::nil())).await;

        let (sender, receiver) = mpsc::channel(100);
        connection.set_inbound_packets_channel(Ok(receiver)).await;
        (connection, sender)
    }

    #[tokio::test]
    async fn test_new_with_example_state_update_packet() {
        let (connection, sender) = create_test_connection().await;
        // TODO find a better way of waiting for the request to be sent before responding
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender
                .send(example_firmware_version_packet())
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender.send(example_state_update_packet()).await.unwrap();
        });
        let device = SoundcoreDevice::<_, TokioFutures>::new(connection)
            .await
            .unwrap();
        let state = device.state().await;
        let sound_modes = state.sound_modes.unwrap();
        assert_eq!(AmbientSoundMode::Normal, sound_modes.ambient_sound_mode);
        assert_eq!(
            NoiseCancelingMode::Transport,
            sound_modes.noise_canceling_mode
        );
        assert!(state.equalizer_configuration.preset_profile().is_none());
        assert_eq!(
            &VolumeAdjustments::new([-6.0, 6.0, 2.3, 4.0, 2.2, 6.0, -0.4, 1.6]).unwrap(),
            state.equalizer_configuration.volume_adjustments(),
        )
    }

    #[tokio::test]
    async fn test_new_with_retry() {
        let (connection, sender) = create_test_connection().await;
        let sender = Arc::new(Mutex::new(sender));
        tokio::spawn({
            let sender = sender.clone();
            async move {
                tokio::time::sleep(Duration::from_millis(1500)).await;
                sender
                    .lock()
                    .await
                    .send(example_firmware_version_packet())
                    .await
                    .unwrap();
                tokio::time::sleep(Duration::from_millis(1500)).await;
                sender
                    .lock()
                    .await
                    .send(example_state_update_packet())
                    .await
                    .unwrap();
            }
        });
        let connection_clone = connection.clone();
        SoundcoreDevice::<_, TokioFutures>::new(connection_clone)
            .await
            .unwrap();
        assert_eq!(0, connection.write_return_queue_length().await);
    }

    #[tokio::test]
    async fn test_new_max_retries() {
        let (connection, _) = create_test_connection().await;

        let connection_clone = connection.clone();
        let result = SoundcoreDevice::<_, TokioFutures>::new(connection_clone).await;
        assert_eq!(true, result.is_err());
    }

    #[tokio::test]
    async fn test_ambient_sound_mode_update_packet() {
        let (connection, sender) = create_test_connection().await;
        connection.push_write_return(Ok(())).await;
        let sender_copy = sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(example_firmware_version_packet())
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(example_state_update_packet())
                .await
                .unwrap();
        });
        let device = SoundcoreDevice::<_, TokioFutures>::new(connection)
            .await
            .unwrap();
        let state = device.state().await;
        let sound_modes = state.sound_modes.unwrap();
        assert_eq!(AmbientSoundMode::Normal, sound_modes.ambient_sound_mode);
        assert_eq!(
            NoiseCancelingMode::Transport,
            sound_modes.noise_canceling_mode
        );

        // alert from device that sound mode changed
        sender
            .send(vec![
                0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x00, 0x01, 0x01, 0x00, 0x20,
            ])
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;

        let state = device.state().await;
        let sound_modes = state.sound_modes.unwrap();

        assert_eq!(
            AmbientSoundMode::NoiseCanceling,
            sound_modes.ambient_sound_mode,
        );
        assert_eq!(
            NoiseCancelingMode::Outdoor,
            sound_modes.noise_canceling_mode,
        );
    }

    #[tokio::test]
    async fn test_set_sound_mode_called_twice() {
        let (connection, sender) = create_test_connection().await;
        // request firmware version packet
        connection.push_write_return(Ok(())).await;
        // request state update packet
        connection.push_write_return(Ok(())).await;
        // first set_sound_modes. second call should not send a packet.
        connection.push_write_return(Ok(())).await;
        // there should be no next write call
        connection
            .push_write_return(Err(crate::Error::MissingData {
                name: "there should not be a second set sound modes packet",
            }))
            .await;

        let sender_copy = sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(example_firmware_version_packet())
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(example_state_update_packet())
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(
                    Packet {
                        command: SetSoundModeOkPacket::header(),
                        body: Vec::new(),
                    }
                    .bytes(),
                )
                .await
                .unwrap();
        });

        let device = SoundcoreDevice::<_, TokioFutures>::new(connection.to_owned())
            .await
            .unwrap();
        let sound_modes = SoundModes {
            custom_noise_canceling: CustomNoiseCanceling::new(10),
            ..Default::default()
        };
        device.set_sound_modes(sound_modes).await.unwrap();
        device.set_sound_modes(sound_modes).await.unwrap();
    }

    #[tokio::test]
    async fn test_set_equalizer_configuration_called_twice() {
        let (connection, sender) = create_test_connection().await;
        // request firmware version packet
        connection.push_write_return(Ok(())).await;
        // request state update packet
        connection.push_write_return(Ok(())).await;
        // first set_equalizer_configuration. second call should not send a packet.
        connection.push_write_return(Ok(())).await;
        // there should be no next write call
        connection
            .push_write_return(Err(crate::Error::MissingData {
                name: "there should not be a second set eq config packet",
            }))
            .await;

        let sender_copy = sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(example_firmware_version_packet())
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(example_state_update_packet())
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(1)).await;
            sender_copy
                .send(
                    Packet {
                        command: SetEqualizerOkPacket::header(),
                        body: Vec::new(),
                    }
                    .bytes(),
                )
                .await
                .unwrap();
        });

        let device = SoundcoreDevice::<_, TokioFutures>::new(connection.to_owned())
            .await
            .unwrap();
        let equalizer_configuration = EqualizerConfiguration::new_custom_profile(
            VolumeAdjustments::new([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]).unwrap(),
        );
        device
            .set_equalizer_configuration(equalizer_configuration.to_owned())
            .await
            .unwrap();
        device
            .set_equalizer_configuration(equalizer_configuration)
            .await
            .unwrap();
    }
}
