use async_trait::async_trait;
use btleplug::{
    api::Characteristic,
    api::{CharPropFlags, Peripheral as _, WriteType},
    platform::Peripheral,
};
use futures::StreamExt;
use tokio::sync::mpsc;
use tracing::{instrument, trace, trace_span, warn};

use crate::soundcore_bluetooth::traits::{
    SoundcoreDeviceConnection, SoundcoreDeviceConnectionError,
};

const WRITE_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: uuid::uuid!("00007777-0000-1000-8000-00805f9b34fb"),
    service_uuid: uuid::uuid!("011cf5da-0000-1000-8000-00805f9b34fb"),
    properties: CharPropFlags::WRITE_WITHOUT_RESPONSE.union(CharPropFlags::WRITE),
};
const NOTIFY_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: uuid::uuid!("00008888-0000-1000-8000-00805F9B34FB"),
    service_uuid: uuid::uuid!("011cf5da-0000-1000-8000-00805f9b34fb"),
    properties: CharPropFlags::READ.union(CharPropFlags::NOTIFY),
};

#[derive(Debug)]
pub struct BtlePlugSoundcoreDeviceConnection {
    peripheral: Peripheral,
    characteristic: Characteristic,
}

impl BtlePlugSoundcoreDeviceConnection {
    pub async fn new(peripheral: Peripheral) -> Result<Self, SoundcoreDeviceConnectionError> {
        peripheral
            .connect()
            .await
            .map_err(SoundcoreDeviceConnectionError::from)?;

        peripheral
            .subscribe(&NOTIFY_CHARACTERISTIC)
            .await
            .map_err(
                |err| SoundcoreDeviceConnectionError::CharacteristicNotFound {
                    uuid: NOTIFY_CHARACTERISTIC.uuid,
                    source: Box::new(err),
                },
            )?;

        let connection = BtlePlugSoundcoreDeviceConnection {
            peripheral,
            characteristic: WRITE_CHARACTERISTIC,
        };

        Ok(connection)
    }
}

#[async_trait]
impl SoundcoreDeviceConnection for BtlePlugSoundcoreDeviceConnection {
    async fn name(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        let maybe_name = self
            .peripheral
            .properties()
            .await
            .map_err(SoundcoreDeviceConnectionError::from)?
            .map(|property| property.local_name);

        match maybe_name {
            Some(Some(name)) => Ok(name),
            _ => Err(SoundcoreDeviceConnectionError::NameNotFound {
                mac_address: self.peripheral.address().to_string(),
            }),
        }
    }

    async fn mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        Ok(self.peripheral.address().to_string())
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_with_response(&self, data: &[u8]) -> Result<(), SoundcoreDeviceConnectionError> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithResponse)
            .await
            .map_err(SoundcoreDeviceConnectionError::from)?;
        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_without_response(
        &self,
        data: &[u8],
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithoutResponse)
            .await
            .map_err(SoundcoreDeviceConnectionError::from)?;
        Ok(())
    }

    async fn inbound_packets_channel(
        &self,
    ) -> Result<mpsc::Receiver<Vec<u8>>, SoundcoreDeviceConnectionError> {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);

        let mut notifications = self
            .peripheral
            .notifications()
            .await
            .map_err(SoundcoreDeviceConnectionError::from)?;

        tokio::spawn(async move {
            let span = trace_span!("inbound_packets_channel async task");
            let _enter = span.enter();
            while let Some(data) = notifications.next().await {
                trace!(event = "btleplug notification", data = ?data);
                if data.uuid == NOTIFY_CHARACTERISTIC.uuid {
                    if let Err(err) = sender.try_send(data.value) {
                        warn!("error forwarding packet to channel: {err}",)
                    }
                }
            }
        });

        Ok(receiver)
    }
}
