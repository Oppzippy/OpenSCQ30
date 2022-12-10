use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{broadcast, mpsc::Receiver, RwLock},
    task::JoinHandle,
    time::timeout,
};
use tracing::{trace, warn};

use crate::{
    packets::outbound::{OutboundPacket, RequestStatePacket},
    soundcore_bluetooth::traits::SoundcoreDeviceConnection,
};
use crate::{
    packets::{
        inbound::InboundPacket,
        outbound::{SetAmbientSoundModePacket, SetEqualizerPacket},
        structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    },
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};

pub struct SoundcoreDevice {
    connection: Arc<dyn SoundcoreDeviceConnection + Send + Sync>,
    state: Arc<RwLock<SoundcoreDeviceState>>,
    inbound_receiver_handle: JoinHandle<()>,
    state_update_sender: broadcast::Sender<SoundcoreDeviceState>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SoundcoreDeviceState {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
    equalizer_configuration: EqualizerConfiguration,
}

impl SoundcoreDeviceState {
    pub fn ambient_sound_mode(self) -> AmbientSoundMode {
        self.ambient_sound_mode
    }
    pub fn noise_canceling_mode(self) -> NoiseCancelingMode {
        self.noise_canceling_mode
    }
    pub fn equalizer_configuration(self) -> EqualizerConfiguration {
        self.equalizer_configuration
    }
}

impl SoundcoreDevice {
    pub async fn new(
        connection: Arc<dyn SoundcoreDeviceConnection + Send + Sync>,
    ) -> Result<Self, SoundcoreDeviceConnectionError> {
        let mut inbound_receiver = connection.inbound_packets_channel().await?;
        let initial_state = Self::fetch_initial_state(&connection, &mut inbound_receiver).await?;

        let current_state_lock = Arc::new(RwLock::new(initial_state));
        let current_state_lock_async = current_state_lock.to_owned();

        let (sender, _) = broadcast::channel(1);

        let sender_copy = sender.to_owned();
        let join_handle = tokio::spawn(async move {
            while let Some(packet_bytes) = inbound_receiver.recv().await {
                match InboundPacket::from_bytes(&packet_bytes) {
                    Some(packet) => {
                        let mut state = current_state_lock_async.write().await;
                        if let Some(new_state) = Self::transform_state_from_packet(&packet, &state)
                        {
                            trace!(event = "state_update", old_state = ?state, new_state = ?new_state);
                            *state = new_state;
                            if let Err(err) = sender_copy.send(new_state) {
                                trace!("failed to broadcast state change: {err}");
                            }
                        }
                    }
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

    fn transform_state_from_packet(
        packet: &InboundPacket,
        state: &SoundcoreDeviceState,
    ) -> Option<SoundcoreDeviceState> {
        match packet {
            InboundPacket::StateUpdate {
                ambient_sound_mode,
                noise_canceling_mode,
                equalizer_configuration,
            } => Some(SoundcoreDeviceState {
                ambient_sound_mode: *ambient_sound_mode,
                noise_canceling_mode: *noise_canceling_mode,
                equalizer_configuration: *equalizer_configuration,
            }),
            InboundPacket::AmbientSoundModeUpdate {
                ambient_sound_mode,
                noise_canceling_mode,
            } => Some(SoundcoreDeviceState {
                ambient_sound_mode: *ambient_sound_mode,
                noise_canceling_mode: *noise_canceling_mode,
                equalizer_configuration: state.equalizer_configuration,
            }),
            InboundPacket::Ok => None,
        }
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
        state.ambient_sound_mode = ambient_sound_mode;
        Ok(())
    }

    pub async fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.state.read().await.ambient_sound_mode
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
        state.noise_canceling_mode = noise_canceling_mode;
        Ok(())
    }

    pub async fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.state.read().await.noise_canceling_mode
    }

    pub async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let mut state = self.state.write().await;
        self.connection
            .write_with_response(&SetEqualizerPacket::new(configuration).bytes())
            .await?;
        state.equalizer_configuration = configuration;
        Ok(())
    }

    pub async fn equalizer_configuration(&self) -> EqualizerConfiguration {
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
