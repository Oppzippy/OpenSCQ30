use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{broadcast, mpsc::Receiver, RwLock},
    task::JoinHandle,
    time::timeout,
};
use tracing::{trace, warn};

use crate::{
    packets::{
        inbound::InboundPacket,
        outbound::{OutboundPacket, RequestStatePacket},
    },
    soundcore_bluetooth::traits::SoundcoreDeviceConnection,
    state::{self, SoundcoreDeviceState},
};
use crate::{
    packets::{
        outbound::{SetAmbientSoundModePacket, SetEqualizerPacket},
        structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    },
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};

pub struct SoundcoreDevice<ConnectionType>
where
    ConnectionType: SoundcoreDeviceConnection + Send + Sync,
{
    connection: Arc<ConnectionType>,
    state: Arc<RwLock<SoundcoreDeviceState>>,
    inbound_receiver_handle: JoinHandle<()>,
    state_update_sender: broadcast::Sender<SoundcoreDeviceState>,
}

impl<ConnectionType> SoundcoreDevice<ConnectionType>
where
    ConnectionType: SoundcoreDeviceConnection + Send + Sync,
{
    pub async fn new(
        connection: Arc<ConnectionType>,
    ) -> Result<Self, SoundcoreDeviceConnectionError> {
        let mut inbound_receiver = connection.inbound_packets_channel().await?;
        let initial_state = Self::fetch_initial_state(&connection, &mut inbound_receiver).await?;

        let current_state_lock = Arc::new(RwLock::new(initial_state));
        let current_state_lock_async = current_state_lock.to_owned();

        let (sender, _) = broadcast::channel(1);

        let sender_copy = sender.to_owned();
        let join_handle = tokio::spawn(async move {
            while let Some(packet_bytes) = inbound_receiver.recv().await {
                match InboundPacket::new(&packet_bytes) {
                    Some(packet) => match state::inbound_packet_to_state_transformer(packet) {
                        Some(transformer) => {
                            let mut state = current_state_lock_async.write().await;
                            let new_state = transformer.transform(&state);
                            if new_state != *state {
                                trace!(event = "state_update", old_state = ?state, new_state = ?new_state);
                                *state = new_state;
                                if let Err(err) = sender_copy.send(new_state) {
                                    trace!("failed to broadcast state change: {err}");
                                }
                            }
                        }
                        None => (),
                    },
                    None => warn!("received unknown packet {:?}", packet_bytes),
                }
            }
        });

        Ok(Self {
            connection,
            state: current_state_lock,
            inbound_receiver_handle: join_handle,
            state_update_sender: sender,
        })
    }

    async fn fetch_initial_state(
        connection: &Arc<ConnectionType>,
        inbound_receiver: &mut Receiver<Vec<u8>>,
    ) -> Result<SoundcoreDeviceState, SoundcoreDeviceConnectionError> {
        for i in 0..3 {
            connection
                .write_without_response(&RequestStatePacket::new().bytes())
                .await?;

            let state_future = async {
                while let Some(packet_bytes) = inbound_receiver.recv().await {
                    match InboundPacket::new(&packet_bytes) {
                        Some(InboundPacket::StateUpdate(packet)) => {
                            return Some(packet.into());
                        }
                        None => warn!("received unknown packet {:?}", packet_bytes),
                        _ => (), // Known packet, but not the one we're looking for
                    };
                }
                None
            };

            match timeout(Duration::from_secs(1), state_future).await {
                Ok(Some(state)) => return Ok(state),
                Err(elapsed) => {
                    warn!(
                        "fetch_initial_state: didn't receive response after {elapsed} on try #{i}"
                    );
                }
                _ => (),
            };
        }
        Err(SoundcoreDeviceConnectionError::NoResponse)
    }

    pub fn subscribe_to_state_updates(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        self.state_update_sender.subscribe()
    }

    pub async fn mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        self.connection.mac_address().await
    }

    pub async fn name(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        self.connection.name().await
    }

    pub async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let noise_canceling_mode = self.noise_canceling_mode().await;
        let mut state = self.state.write().await;
        self.connection
            .write_with_response(
                &SetAmbientSoundModePacket::new(ambient_sound_mode, noise_canceling_mode).bytes(),
            )
            .await?;
        *state = state.with_ambient_sound_mode(ambient_sound_mode);
        Ok(())
    }

    pub async fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.state.read().await.ambient_sound_mode()
    }

    pub async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let ambient_sound_mode = self.ambient_sound_mode().await;
        let mut state = self.state.write().await;
        // It will bug and put us in noise canceling mode without changing the ambient sound mode id if we change the
        // noise canceling mode with the ambient sound mode being normal or transparency. To work around this, we must
        // set the ambient sound mode to Noise Canceling, and then change it back.
        self.connection
            .write_with_response(
                &SetAmbientSoundModePacket::new(
                    AmbientSoundMode::NoiseCanceling,
                    noise_canceling_mode,
                )
                .bytes(),
            )
            .await?;

        // Set us back to the ambient sound mode we were originally in
        if ambient_sound_mode != AmbientSoundMode::NoiseCanceling {
            self.connection
                .write_with_response(
                    &SetAmbientSoundModePacket::new(ambient_sound_mode, noise_canceling_mode)
                        .bytes(),
                )
                .await?;
        }
        *state = state.with_noise_canceling_mode(noise_canceling_mode);
        Ok(())
    }

    pub async fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.state.read().await.noise_canceling_mode()
    }

    pub async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let mut state = self.state.write().await;
        self.connection
            .write_with_response(&SetEqualizerPacket::new(configuration).bytes())
            .await?;
        *state = state.with_equalizer_configuration(configuration);
        Ok(())
    }

    pub async fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.state.read().await.equalizer_configuration()
    }
}

impl<ConnectionType> Drop for SoundcoreDevice<ConnectionType>
where
    ConnectionType: SoundcoreDeviceConnection + Send + Sync,
{
    fn drop(&mut self) {
        self.inbound_receiver_handle.abort();
    }
}

impl<ConnectionType> std::fmt::Debug for SoundcoreDevice<ConnectionType>
where
    ConnectionType: SoundcoreDeviceConnection + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoundcoreDevice").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use tokio::sync::mpsc;

    use super::SoundcoreDevice;
    use crate::{
        packets::structures::{
            AmbientSoundMode, EqualizerBandOffsets, EqualizerConfiguration, NoiseCancelingMode,
        },
        soundcore_bluetooth::stub::StubSoundcoreDeviceConnection,
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

    async fn create_test_connection() -> (Arc<StubSoundcoreDeviceConnection>, mpsc::Sender<Vec<u8>>)
    {
        let connection = Arc::new(StubSoundcoreDeviceConnection::new());
        connection
            .set_name_return(Ok("Soundcore Q30".to_string()))
            .await;
        connection
            .set_mac_address_return(Ok("00:00:00:00:00:00".to_string()))
            .await;

        let (sender, receiver) = mpsc::channel(100);
        connection.set_inbound_packets_channel(Ok(receiver)).await;
        (connection, sender)
    }

    #[tokio::test]
    async fn test_new_with_example_state_update_packet() {
        let (connection, sender) = create_test_connection().await;
        connection.push_write_return(Ok(())).await;
        tokio::spawn(async move {
            sender.send(example_state_update_packet()).await.unwrap();
        });
        let device = SoundcoreDevice::new(connection).await.unwrap();
        assert_eq!(AmbientSoundMode::Normal, device.ambient_sound_mode().await);
        assert_eq!(
            NoiseCancelingMode::Transport,
            device.noise_canceling_mode().await
        );
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            device.equalizer_configuration().await
        )
    }

    #[tokio::test]
    async fn test_new_with_retry() {
        let (connection, sender) = create_test_connection().await;
        connection.push_write_return(Ok(())).await;
        connection.push_write_return(Ok(())).await;
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1500)).await;
            sender.send(example_state_update_packet()).await.unwrap();
        });
        let connection_clone = connection.clone();
        SoundcoreDevice::new(connection_clone).await.unwrap();
        assert_eq!(0, connection.write_return_queue_length().await);
    }

    #[tokio::test]
    async fn test_new_max_retries() {
        let (connection, _) = create_test_connection().await;
        // for the purposes of this test, we don't care how many times it retries. we only care that it stops at some point.
        for _ in 0..100 {
            connection.push_write_return(Ok(())).await;
        }

        let connection_clone = connection.clone();
        let result = SoundcoreDevice::new(connection_clone).await;
        assert_eq!(true, result.is_err());
    }

    #[tokio::test]
    async fn test_ambient_sound_mode_update_packet() {
        let (connection, sender) = create_test_connection().await;
        connection.push_write_return(Ok(())).await;
        let sender_copy = sender.clone();
        tokio::spawn(async move {
            sender_copy
                .send(example_state_update_packet())
                .await
                .unwrap();
        });
        let device = SoundcoreDevice::new(connection).await.unwrap();
        assert_eq!(AmbientSoundMode::Normal, device.ambient_sound_mode().await);
        assert_eq!(
            NoiseCancelingMode::Transport,
            device.noise_canceling_mode().await
        );

        tokio::spawn(async move {
            sender
                .send(vec![
                    0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x00, 0x01, 0x01, 0x00,
                    0x20,
                ])
                .await
                .unwrap();
        });
        // wait for the packet to be handled asynchronously
        tokio::time::sleep(Duration::from_millis(100)).await;

        assert_eq!(
            AmbientSoundMode::NoiseCanceling,
            device.ambient_sound_mode().await
        );
        assert_eq!(
            NoiseCancelingMode::Outdoor,
            device.noise_canceling_mode().await
        );
    }
}
