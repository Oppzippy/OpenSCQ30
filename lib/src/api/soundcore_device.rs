use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc::Receiver, RwLock},
    task::JoinHandle,
    time::timeout,
};
use tracing::{debug, warn};

use crate::{
    packets::outbound::{
        outbound_packet::OutboundPacket, request_state_packet::RequestStatePacket,
    },
    soundcore_bluetooth::traits::soundcore_device_connection::SoundcoreDeviceConnection,
};
use crate::{
    packets::{
        inbound::inbound_packet::InboundPacket,
        outbound::{
            set_ambient_mode::SetAmbientSoundModePacket, set_equalizer::SetEqualizerPacket,
        },
        structures::{
            ambient_sound_mode::AmbientSoundMode, equalizer_configuration::EqualizerConfiguration,
            noise_canceling_mode::NoiseCancelingMode,
        },
    },
    soundcore_bluetooth::traits::soundcore_device_connection_error::SoundcoreDeviceConnectionError,
};

pub struct SoundcoreDevice {
    connection: Arc<dyn SoundcoreDeviceConnection + Send + Sync>,
    state: Arc<RwLock<SoundcoreDeviceState>>,
    inbound_receiver_handle: JoinHandle<()>,
}

struct SoundcoreDeviceState {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
    equalizer_configuration: EqualizerConfiguration,
}

impl SoundcoreDevice {
    pub async fn new(
        connection: Arc<dyn SoundcoreDeviceConnection + Send + Sync>,
    ) -> Result<Self, SoundcoreDeviceConnectionError> {
        let mut inbound_receiver = connection.inbound_packets_channel().await?;
        let initial_state = Self::get_state(&connection, &mut inbound_receiver).await?;

        let current_state_lock = Arc::new(RwLock::new(initial_state));
        let current_state_lock_async = current_state_lock.to_owned();

        let join_handle = tokio::spawn(async move {
            while let Some(packet_bytes) = inbound_receiver.recv().await {
                match InboundPacket::from_bytes(&packet_bytes) {
                    Some(packet) => {
                        let mut state = current_state_lock_async.write().await;
                        Self::on_packet_received(&packet, &mut state);
                    }
                    None => debug!(
                        "received unknown packet {}",
                        packet_bytes
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                }
            }
        });

        Ok(Self {
            connection,
            state: current_state_lock,
            inbound_receiver_handle: join_handle,
        })
    }

    async fn get_state(
        connection: &Arc<dyn SoundcoreDeviceConnection + Send + Sync>,
        inbound_receiver: &mut Receiver<Vec<u8>>,
    ) -> Result<SoundcoreDeviceState, SoundcoreDeviceConnectionError> {
        for i in 0..3 {
            connection
                .write_without_response(&RequestStatePacket::new().bytes())
                .await?;

            let state_future = async {
                while let Some(packet_bytes) = inbound_receiver.recv().await {
                    match InboundPacket::from_bytes(&packet_bytes) {
                        Some(InboundPacket::StateUpdate {
                            ambient_sound_mode,
                            noise_canceling_mode,
                            equalizer_configuration,
                        }) => {
                            return Some(SoundcoreDeviceState {
                                ambient_sound_mode,
                                noise_canceling_mode,
                                equalizer_configuration,
                            });
                        }
                        None => debug!(
                            "received unknown packet {}",
                            packet_bytes
                                .iter()
                                .map(|v| v.to_string())
                                .collect::<Vec<String>>()
                                .join(" ")
                        ),
                        _ => (),
                    };
                }
                None
            };

            match timeout(Duration::from_secs(1), state_future).await {
                Ok(Some(state)) => return Ok(state),
                Err(elapsed) => {
                    warn!("get_state: didn't receive response after {elapsed} on try #{i}");
                }
                _ => (),
            };
        }
        Err(SoundcoreDeviceConnectionError::NoResponse)
    }

    fn on_packet_received(packet: &InboundPacket, state: &mut SoundcoreDeviceState) {
        match packet {
            InboundPacket::StateUpdate {
                ambient_sound_mode,
                noise_canceling_mode,
                equalizer_configuration,
            } => {
                state.ambient_sound_mode = *ambient_sound_mode;
                state.noise_canceling_mode = *noise_canceling_mode;
                state.equalizer_configuration = *equalizer_configuration;
            }
            InboundPacket::AmbientSoundModeUpdate {
                ambient_sound_mode,
                noise_canceling_mode,
            } => {
                state.ambient_sound_mode = *ambient_sound_mode;
                state.noise_canceling_mode = *noise_canceling_mode;
            }
        }
    }

    pub async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let noise_canceling_mode = self.get_noise_canceling_mode().await;
        self.connection
            .write_with_response(
                &SetAmbientSoundModePacket::new(ambient_sound_mode, noise_canceling_mode).bytes(),
            )
            .await?;
        Ok(())
    }

    pub async fn get_ambient_sound_mode(&self) -> AmbientSoundMode {
        self.state.read().await.ambient_sound_mode
    }

    pub async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let ambient_sound_mode = self.get_ambient_sound_mode().await;
        self.connection
            .write_with_response(
                &SetAmbientSoundModePacket::new(ambient_sound_mode, noise_canceling_mode).bytes(),
            )
            .await?;
        Ok(())
    }

    pub async fn get_noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.state.read().await.noise_canceling_mode
    }

    pub async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        self.connection
            .write_with_response(&SetEqualizerPacket::new(configuration).bytes())
            .await?;
        Ok(())
    }

    pub async fn get_equalizer_configuration(&self) -> EqualizerConfiguration {
        self.state.read().await.equalizer_configuration
    }
}

impl Drop for SoundcoreDevice {
    fn drop(&mut self) {
        self.inbound_receiver_handle.abort();
    }
}

impl std::fmt::Debug for SoundcoreDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoundcoreDevice").finish()
    }
}
