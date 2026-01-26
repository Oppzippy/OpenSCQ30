use std::{panic::Location, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::watch;

use crate::{
    api::{
        connection::{ConnectionDescriptor, ConnectionStatus, RfcommBackend, RfcommConnection},
        device::{self, OpenSCQ30Device, OpenSCQ30DeviceRegistry},
        settings::{CategoryId, Setting, SettingId, Value},
    },
    connection::RfcommServiceSelectionStrategy,
    devices::{
        DeviceModel,
        soundcore::{
            self,
            common::packet::{
                self, Command, PacketIOController,
                outbound::{RequestState, ToPacket},
            },
        },
    },
};

pub struct SoundcoreDevelopmentDeviceRegistry<B: RfcommBackend> {
    backend: B,
}

impl<B> SoundcoreDevelopmentDeviceRegistry<B>
where
    B: RfcommBackend,
{
    pub fn new(backend: B) -> Self {
        Self { backend }
    }
}

#[async_trait]
impl<B> OpenSCQ30DeviceRegistry for SoundcoreDevelopmentDeviceRegistry<B>
where
    B: RfcommBackend + Send + Sync + 'static,
{
    async fn devices(&self) -> device::Result<Vec<ConnectionDescriptor>> {
        self.backend
            .devices()
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>())
            .map_err(Into::into)
    }

    async fn connect(
        &self,
        mac_address: MacAddr6,
    ) -> device::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        let connection = self
            .backend
            .connect(
                mac_address,
                RfcommServiceSelectionStrategy::Dynamic(|service_uuids| {
                    service_uuids
                        .into_iter()
                        .find(soundcore::is_soundcore_vendor_rfcomm_uuid)
                        .unwrap_or(soundcore::RFCOMM_UUID)
                }),
            )
            .await?;
        let device = SoundcoreDevelopmentDevice::<B>::new(Arc::new(connection)).await?;
        Ok(Arc::new(device))
    }
}

pub struct SoundcoreDevelopmentDevice<B>
where
    B: RfcommBackend,
{
    packet_io: PacketIOController<B::ConnectionType>,
    backend: Arc<B::ConnectionType>,
    state_update_packet: Option<packet::Inbound>,
    changes_signal: watch::Sender<()>,
}

impl<B> SoundcoreDevelopmentDevice<B>
where
    B: RfcommBackend,
{
    async fn new(connection: Arc<B::ConnectionType>) -> device::Result<Self> {
        let (packet_io, _packet_receiver) =
            PacketIOController::new(connection.to_owned(), packet::ChecksumKind::Suffix).await?;
        let state_update_packet = packet_io
            .send_with_response(&RequestState::default().to_packet())
            .await
            .ok();
        Ok(Self {
            packet_io,
            backend: connection,
            state_update_packet,
            changes_signal: watch::channel(()).0,
        })
    }
}

#[async_trait]
impl<B> OpenSCQ30Device for SoundcoreDevelopmentDevice<B>
where
    B: RfcommBackend,
{
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.backend.connection_status()
    }

    fn model(&self) -> DeviceModel {
        DeviceModel::SoundcoreDevelopment
    }

    fn categories(&self) -> Vec<CategoryId> {
        vec![CategoryId::DeviceInformation]
    }

    fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId> {
        if *category_id == CategoryId::DeviceInformation {
            vec![SettingId::StateUpdatePacket, SettingId::SendPacket]
        } else {
            Vec::new()
        }
    }

    fn setting(&self, setting_id: &SettingId) -> Option<Setting> {
        match setting_id {
            SettingId::StateUpdatePacket => {
                let text = format!("{:?}", self.state_update_packet);
                Some(Setting::Information {
                    value: text.to_owned(),
                    translated_value: text,
                })
            }
            SettingId::SendPacket => Some(Setting::ImportString {
                confirmation_message: None,
            }),
            _ => None,
        }
    }

    fn watch_for_changes(&self) -> watch::Receiver<()> {
        self.changes_signal.subscribe()
    }

    async fn set_setting_values(
        &self,
        setting_values: Vec<(SettingId, Value)>,
    ) -> device::Result<()> {
        for (setting_id, value) in setting_values {
            if setting_id == SettingId::SendPacket {
                let mut data = value
                    .try_as_str()
                    .map_err(|err| device::Error::Other {
                        source: Box::new(err),
                        location: Location::caller(),
                    })?
                    .split(',')
                    .map(|item| {
                        let item = item.trim_ascii();
                        if let Some(hex_number) = item.strip_prefix("0x") {
                            u8::from_str_radix(hex_number, 16)
                        } else {
                            item.parse::<u8>()
                        }
                    })
                    .collect::<Result<Vec<u8>, _>>()
                    .map_err(|err| device::Error::Other {
                        source: Box::new(err),
                        location: Location::caller(),
                    })?;

                if data.len() < 2 {
                    return Err(device::Error::Other {
                        source: Box::new(DevelopmentDeviceError::MissingCommand),
                        location: Location::caller(),
                    });
                }

                let body = data.split_off(2);
                let command = Command(data.try_into().unwrap());

                self.packet_io
                    .send_with_response(&packet::Outbound::new(command, body))
                    .await
                    .unwrap();
            }
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
enum DevelopmentDeviceError {
    #[error("data length must be at least 2, since the first 2 bytes are used as the command")]
    MissingCommand,
}
