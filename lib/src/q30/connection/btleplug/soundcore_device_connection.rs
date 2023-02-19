use async_trait::async_trait;
use btleplug::{
    api::Characteristic,
    api::{CharPropFlags, Peripheral as _, WriteType},
    platform::Peripheral,
};
use futures::StreamExt;
use tokio::sync::mpsc::{self, error::TrySendError};
use tracing::{instrument, trace, trace_span, warn};

use crate::{api::connection::SoundcoreDeviceConnection, soundcore_device_utils};

const WRITE_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: soundcore_device_utils::WRITE_CHARACTERISTIC_UUID,
    service_uuid: soundcore_device_utils::SERVICE_UUID,
    properties: CharPropFlags::WRITE_WITHOUT_RESPONSE.union(CharPropFlags::WRITE),
};
const NOTIFY_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: soundcore_device_utils::READ_CHARACTERISTIC_UUID,
    service_uuid: soundcore_device_utils::SERVICE_UUID,
    properties: CharPropFlags::READ.union(CharPropFlags::NOTIFY),
};

#[derive(Debug)]
pub struct BtlePlugSoundcoreDeviceConnection {
    peripheral: Peripheral,
    characteristic: Characteristic,
}

impl BtlePlugSoundcoreDeviceConnection {
    pub async fn new(peripheral: Peripheral) -> crate::Result<Self> {
        peripheral.connect().await?;
        peripheral.discover_services().await?;

        peripheral
            .subscribe(&NOTIFY_CHARACTERISTIC)
            .await
            .map_err(|err| crate::Error::CharacteristicNotFound {
                uuid: NOTIFY_CHARACTERISTIC.uuid,
                source: Box::new(err),
            })?;

        let connection = BtlePlugSoundcoreDeviceConnection {
            peripheral,
            characteristic: WRITE_CHARACTERISTIC,
        };

        Ok(connection)
    }
}

#[async_trait]
impl SoundcoreDeviceConnection for BtlePlugSoundcoreDeviceConnection {
    async fn name(&self) -> crate::Result<String> {
        let maybe_name = self
            .peripheral
            .properties()
            .await?
            .map(|property| property.local_name);

        match maybe_name {
            Some(Some(name)) => Ok(name),
            _ => Err(crate::Error::NameNotFound {
                mac_address: self.peripheral.address().to_string(),
            }),
        }
    }

    async fn mac_address(&self) -> crate::Result<String> {
        Ok(self.peripheral.address().to_string())
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithResponse)
            .await?;
        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);

        let mut notifications = self.peripheral.notifications().await?;

        tokio::spawn(async move {
            let span = trace_span!("inbound_packets_channel async task");
            let _enter = span.enter();
            while let Some(data) = notifications.next().await {
                trace!(event = "btleplug notification", data = ?data);
                if data.uuid == NOTIFY_CHARACTERISTIC.uuid {
                    if let Err(err) = sender.try_send(data.value) {
                        if let TrySendError::Closed(_) = err {
                            break;
                        }
                        warn!("error forwarding packet to channel: {err}",)
                    }
                }
            }
        });

        Ok(receiver)
    }
}
