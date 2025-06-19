use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::watch;

use crate::{
    api::{
        connection::{ConnectionDescriptor, ConnectionStatus, RfcommBackend, RfcommConnection},
        device::{self, OpenSCQ30Device, OpenSCQ30DeviceRegistry},
        settings::{CategoryId, Setting, SettingId, Value},
    },
    device_utils,
    devices::{
        DeviceModel,
        soundcore::standard::{
            demo::DemoConnectionRegistry,
            packets::{
                Packet, inbound::state_update_packet, outbound::RequestStatePacket,
                packet_io_controller::PacketIOController,
            },
        },
    },
    storage,
};

pub fn device_registry<B>(
    backend: B,
    _database: Arc<storage::OpenSCQ30Database>,
    _device_model: DeviceModel,
) -> SoundcoreDevelopmentDeviceRegistry<B>
where
    B: RfcommBackend + Send + Sync + 'static,
{
    SoundcoreDevelopmentDeviceRegistry::new(backend)
}

pub fn demo_device_registry(
    _database: Arc<storage::OpenSCQ30Database>,
    device_model: DeviceModel,
) -> SoundcoreDevelopmentDeviceRegistry<
    crate::devices::soundcore::standard::demo::DemoConnectionRegistry,
> {
    SoundcoreDevelopmentDeviceRegistry::new(DemoConnectionRegistry::new(
        device_model,
        HashMap::from([(state_update_packet::COMMAND, vec![1, 2, 3])]),
    ))
}

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
            .connect(mac_address, |addr| {
                addr.into_iter()
                    .find(device_utils::is_soundcore_vendor_rfcomm_uuid)
                    .unwrap_or(device_utils::RFCOMM_UUID)
            })
            .await?;
        let device = SoundcoreDevelopmentDevice::<B>::new(Arc::new(connection)).await?;
        Ok(Arc::new(device))
    }
}

pub struct SoundcoreDevelopmentDevice<B>
where
    B: RfcommBackend,
{
    backend: Arc<B::ConnectionType>,
    state_update_packet: Option<Packet>,
    changes_signal: watch::Sender<()>,
}

impl<B> SoundcoreDevelopmentDevice<B>
where
    B: RfcommBackend,
{
    async fn new(connection: Arc<B::ConnectionType>) -> device::Result<Self> {
        let (packet_io, _packet_receiver) = PacketIOController::new(connection.to_owned()).await?;
        let state_update_packet = packet_io.send(&RequestStatePacket::new().into()).await.ok();
        Ok(Self {
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
            vec![SettingId::StateUpdatePacket]
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
            _ => None,
        }
    }

    fn watch_for_changes(&self) -> watch::Receiver<()> {
        self.changes_signal.subscribe()
    }

    async fn set_setting_values(
        &self,
        _setting_values: Vec<(SettingId, Value)>,
    ) -> device::Result<()> {
        panic!()
    }
}
