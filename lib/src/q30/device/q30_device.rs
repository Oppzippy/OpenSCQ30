use std::{rc::Rc, sync::Arc, time::Duration};

use async_trait::async_trait;
use futures::FutureExt;
use macaddr::MacAddr6;
use tokio::sync::{broadcast, mpsc::Receiver, watch, RwLock};
use tracing::{trace, warn};

use crate::{
    api::connection::{Connection, ConnectionStatus},
    futures::{sleep, spawn, JoinHandle},
    packets::{
        outbound::{OutboundPacketBytes, SetEqualizerPacket, SetSoundModePacket},
        structures::{AmbientSoundMode, DeviceFeatureFlags, EqualizerConfiguration, SoundModes},
    },
};
use crate::{
    api::{self},
    packets::{inbound::InboundPacket, outbound::RequestStatePacket},
    state::{self, DeviceState},
};

pub struct Q30Device<ConnectionType>
where
    ConnectionType: Connection,
{
    connection: Rc<ConnectionType>,
    state: Arc<RwLock<DeviceState>>,
    join_handle: Box<dyn JoinHandle>,
    state_update_sender: broadcast::Sender<DeviceState>,
}

impl<ConnectionType> Q30Device<ConnectionType>
where
    ConnectionType: Connection,
{
    pub async fn new(connection: Rc<ConnectionType>) -> crate::Result<Self> {
        let mut inbound_receiver = connection.inbound_packets_channel().await?;
        let initial_state = Self::fetch_initial_state(&connection, &mut inbound_receiver).await?;

        let current_state_lock = Arc::new(RwLock::new(initial_state));
        let current_state_lock_async = current_state_lock.to_owned();

        let (sender, _) = broadcast::channel(1);

        let sender_copy = sender.to_owned();
        let join_handle = spawn(async move {
            while let Some(packet_bytes) = inbound_receiver.recv().await {
                match InboundPacket::new(&packet_bytes) {
                    Ok(packet) => {
                        if let Some(transformer) =
                            state::inbound_packet_to_state_transformer(packet)
                        {
                            let mut state = current_state_lock_async.write().await;
                            let new_state = transformer.transform(&state);
                            if new_state != *state {
                                trace!(event = "state_update", old_state = ?state, new_state = ?new_state);
                                *state = new_state.clone();
                                if let Err(err) = sender_copy.send(new_state) {
                                    trace!("failed to broadcast state change: {err}");
                                }
                            }
                        }
                    }
                    Err(err) => warn!("failed to parse packet: {err:?}"),
                }
            }
        });

        Ok(Self {
            connection,
            state: current_state_lock,
            join_handle: Box::new(join_handle),
            state_update_sender: sender,
        })
    }

    async fn fetch_initial_state(
        connection: &ConnectionType,
        inbound_receiver: &mut Receiver<Vec<u8>>,
    ) -> crate::Result<DeviceState> {
        for i in 0..3 {
            connection
                .write_without_response(&RequestStatePacket::new().bytes())
                .await?;

            let state_future = async {
                while let Some(packet_bytes) = inbound_receiver.recv().await {
                    match InboundPacket::new(&packet_bytes) {
                        Ok(InboundPacket::StateUpdate(packet)) => {
                            return Some(packet.into());
                        }
                        Err(err) => warn!("failed to parse packet: {err:?}"),
                        _ => (), // Known packet, but not the one we're looking for
                    };
                }
                None
            };

            futures::select! {
                state = state_future.fuse() => if let Some(state) = state { return Ok(state) },
                _ = sleep(Duration::from_secs(1)).fuse() =>
                    warn!( "fetch_initial_state: didn't receive response after 1 second on try #{i}"),
            }
        }
        Err(crate::Error::NoResponse)
    }
}

#[async_trait(?Send)]
impl<ConnectionType> api::device::Device for Q30Device<ConnectionType>
where
    ConnectionType: Connection,
{
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.state_update_sender.subscribe()
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

    async fn state(&self) -> DeviceState {
        self.state.read().await.clone()
    }

    async fn set_sound_modes(&self, sound_modes: SoundModes) -> crate::Result<()> {
        let mut state = self.state.write().await;
        let Some(prev_sound_modes) = state.sound_modes else {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "sound modes",
            });
        };
        if prev_sound_modes == sound_modes {
            return Ok(());
        }

        // It will bug and put us in noise canceling mode without changing the ambient sound mode id if we change the
        // noise canceling mode with the ambient sound mode being normal or transparency. To work around this, we must
        // set the ambient sound mode to Noise Canceling, and then change it back.
        let needs_noise_canceling = prev_sound_modes.ambient_sound_mode
            != AmbientSoundMode::NoiseCanceling
            && prev_sound_modes.noise_canceling_mode != sound_modes.noise_canceling_mode;
        if needs_noise_canceling {
            self.connection
                .write_with_response(
                    &SetSoundModePacket {
                        ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
                        noise_canceling_mode: prev_sound_modes.noise_canceling_mode,
                        transparency_mode: prev_sound_modes.transparency_mode,
                        custom_noise_canceling: prev_sound_modes.custom_noise_canceling,
                    }
                    .bytes(),
                )
                .await?;
        }

        // If we need to temporarily be in noise canceling mode to work around the bug, set all fields besides
        // ambient_sound_mode. Otherwise, we set all fields in one go.
        self.connection
            .write_with_response(
                &SetSoundModePacket {
                    ambient_sound_mode: if needs_noise_canceling {
                        AmbientSoundMode::NoiseCanceling
                    } else {
                        sound_modes.ambient_sound_mode
                    },
                    noise_canceling_mode: sound_modes.noise_canceling_mode,
                    transparency_mode: sound_modes.transparency_mode,
                    custom_noise_canceling: sound_modes.custom_noise_canceling,
                }
                .bytes(),
            )
            .await?;

        // Switch to the target sound mode if we didn't do it in the previous step.
        // If the target sound mode is noise canceling, we already set it to that, so no change needed.
        if needs_noise_canceling
            && sound_modes.ambient_sound_mode != AmbientSoundMode::NoiseCanceling
        {
            self.connection
                .write_with_response(
                    &SetSoundModePacket {
                        ambient_sound_mode: sound_modes.ambient_sound_mode,
                        noise_canceling_mode: sound_modes.noise_canceling_mode,
                        transparency_mode: sound_modes.transparency_mode,
                        custom_noise_canceling: sound_modes.custom_noise_canceling,
                    }
                    .bytes(),
                )
                .await?;
        }
        *state = DeviceState {
            sound_modes: Some(sound_modes),
            ..state.clone()
        };
        Ok(())
    }

    async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<()> {
        let mut state = self.state.write().await;
        if equalizer_configuration == state.equalizer_configuration {
            return Ok(());
        }

        let left_channel = equalizer_configuration;
        let right_channel = if state
            .feature_flags
            .contains(DeviceFeatureFlags::TWO_CHANNEL_EQUALIZER)
        {
            Some(equalizer_configuration)
        } else {
            None
        };

        self.connection
            .write_with_response(&SetEqualizerPacket::new(left_channel, right_channel).bytes())
            .await?;

        *state = DeviceState {
            equalizer_configuration,
            ..state.clone()
        };
        Ok(())
    }
}

impl<ConnectionType> Drop for Q30Device<ConnectionType>
where
    ConnectionType: Connection,
{
    fn drop(&mut self) {
        self.join_handle.abort();
    }
}

impl<ConnectionType> std::fmt::Debug for Q30Device<ConnectionType>
where
    ConnectionType: Connection,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoundcoreDevice").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, time::Duration};

    use macaddr::MacAddr6;
    use tokio::sync::mpsc;

    use super::Q30Device;
    use crate::{
        api::device::Device,
        packets::structures::{
            AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
            SoundModes, VolumeAdjustments,
        },
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

    async fn create_test_connection() -> (Rc<StubConnection>, mpsc::Sender<Vec<u8>>) {
        let connection = Rc::new(StubConnection::new());
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
        connection.push_write_return(Ok(())).await;
        tokio::spawn(async move {
            sender.send(example_state_update_packet()).await.unwrap();
        });
        let device = Q30Device::new(connection).await.unwrap();
        let state = device.state().await;
        let sound_modes = state.sound_modes.unwrap();
        assert_eq!(AmbientSoundMode::Normal, sound_modes.ambient_sound_mode);
        assert_eq!(
            NoiseCancelingMode::Transport,
            sound_modes.noise_canceling_mode
        );
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            state.equalizer_configuration
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
        Q30Device::new(connection_clone).await.unwrap();
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
        let result = Q30Device::new(connection_clone).await;
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
        let device = Q30Device::new(connection).await.unwrap();
        let state = device.state().await;
        let sound_modes = state.sound_modes.unwrap();
        assert_eq!(AmbientSoundMode::Normal, sound_modes.ambient_sound_mode);
        assert_eq!(
            NoiseCancelingMode::Transport,
            sound_modes.noise_canceling_mode
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
        // request state update packet
        connection.push_write_return(Ok(())).await;
        // first set_sound_modes. second call should not send a packet.
        connection.push_write_return(Ok(())).await;
        sender.send(example_state_update_packet()).await.unwrap();

        let device = Q30Device::new(connection.to_owned()).await.unwrap();
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
        // request state update packet
        connection.push_write_return(Ok(())).await;
        // first set_equalizer_configuration. second call should not send a packet.
        connection.push_write_return(Ok(())).await;
        sender.send(example_state_update_packet()).await.unwrap();

        let device = Q30Device::new(connection.to_owned()).await.unwrap();
        let equalizer_configuration =
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                0, 10, 20, 30, 40, 50, 60, 70,
            ]));
        device
            .set_equalizer_configuration(equalizer_configuration)
            .await
            .unwrap();
        device
            .set_equalizer_configuration(equalizer_configuration)
            .await
            .unwrap();
    }
}
