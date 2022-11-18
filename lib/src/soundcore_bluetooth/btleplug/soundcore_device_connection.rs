use std::error::Error;

use async_trait::async_trait;
use btleplug::{
    api::Characteristic,
    api::{CharPropFlags, Peripheral as _, WriteType},
    platform::Peripheral,
};
use futures::StreamExt;
use tokio::sync::mpsc;
use tracing::{debug, warn};

use crate::{
    packets::inbound::inbound_packet::InboundPacket,
    soundcore_bluetooth::traits::{
        soundcore_device_connection::SoundcoreDeviceConnection,
        soundcore_device_connection_error::SoundcoreDeviceError,
    },
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

pub struct BtlePlugSoundcoreDeviceConnection {
    peripheral: Peripheral,
    characteristic: Characteristic,
}

impl BtlePlugSoundcoreDeviceConnection {
    pub async fn new(peripheral: Peripheral) -> Result<Self, SoundcoreDeviceError> {
        match peripheral.subscribe(&NOTIFY_CHARACTERISTIC).await {
            Ok(_) => (),
            Err(_) => {
                return Err(SoundcoreDeviceError::CharacteristicNotFound(
                    NOTIFY_CHARACTERISTIC.uuid.to_string(),
                ));
            }
        };

        let connection = BtlePlugSoundcoreDeviceConnection {
            peripheral,
            characteristic: WRITE_CHARACTERISTIC,
        };

        Ok(connection)
    }
}

#[async_trait]
impl SoundcoreDeviceConnection for BtlePlugSoundcoreDeviceConnection {
    async fn get_name(&self) -> Result<String, Box<dyn Error>> {
        let maybe_name = self
            .peripheral
            .properties()
            .await?
            .map(|property| property.local_name);

        match maybe_name {
            Some(Some(name)) => Ok(name),
            _ => Err(Box::new(SoundcoreDeviceError::NameNotFound(
                self.peripheral.address().to_string(),
            ))),
        }
    }

    async fn get_mac_address(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.peripheral.address().to_string())
    }

    async fn write_with_response(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithResponse)
            .await?;
        Ok(())
    }

    async fn write_without_response(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    async fn inbound_packets_channel(
        &self,
    ) -> Result<mpsc::Receiver<InboundPacket>, SoundcoreDeviceError> {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);

        let mut notifications = self.peripheral.notifications().await.unwrap();

        tokio::spawn(async move {
            while let Some(data) = notifications.next().await {
                if data.uuid == NOTIFY_CHARACTERISTIC.uuid {
                    match InboundPacket::from_bytes(&data.value) {
                        Some(packet) => match sender.try_send(packet) {
                            Ok(_) => (),
                            Err(err) => warn!(
                                "inbound_packets_channel: error sending packet to channel: {}",
                                err
                            ),
                        },
                        None => {
                            debug!(
                                "received unknown packet {}",
                                data.value
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            )
                        }
                    };
                }
            }
        });

        Ok(receiver)
    }
}
