use std::{collections::HashSet, str::FromStr, sync::Arc, thread};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::oneshot;
use windows::Devices::Bluetooth::{self, BluetoothConnectionStatus, BluetoothDevice};

use crate::api::connection::ConnectionRegistry;

use super::{WindowsConnection, WindowsConnectionDescriptor, WindowsMacAddress};

pub struct WindowsConnectionRegistry {}

impl WindowsConnectionRegistry {
    pub async fn new() -> crate::Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl ConnectionRegistry for WindowsConnectionRegistry {
    type ConnectionType = WindowsConnection;
    type DescriptorType = WindowsConnectionDescriptor;

    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>> {
        let (result_sender, result_receiver) = oneshot::channel();
        thread::spawn(move || {
            let result = (|| {
                let devices =
                    windows::Devices::Enumeration::DeviceInformation::FindAllAsyncAqsFilter(
                        &Bluetooth::BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
                            BluetoothConnectionStatus::Connected,
                        )?,
                    )?
                    .get()?;
                let descriptors = devices
                    .into_iter()
                    .map(|device| {
                        let id = device.Id()?;
                        let bluetooth_device = BluetoothDevice::FromIdAsync(&id)?.get()?;
                        let mac_address =
                            MacAddr6::from_windows_u64(bluetooth_device.BluetoothAddress()?);

                        Ok(WindowsConnectionDescriptor::new(
                            bluetooth_device.Name()?.to_string(),
                            mac_address.to_string(),
                        )) as crate::Result<WindowsConnectionDescriptor>
                    })
                    .filter_map(|result| match result {
                        Ok(descriptor) => Some(descriptor),
                        Err(err) => {
                            tracing::warn!("error creating device descriptor: {}", err);
                            None
                        }
                    })
                    .collect();
                Ok(descriptors) as crate::Result<HashSet<Self::DescriptorType>>
            })();
            result_sender.send(result).unwrap();
        });
        result_receiver.await.map_err(|err| crate::Error::Other {
            source: Box::new(err),
        })?
    }

    async fn connection(
        &self,
        mac_address: &str,
    ) -> crate::Result<Option<Arc<Self::ConnectionType>>> {
        let mac_address =
            MacAddr6::from_str(mac_address).map_err(|err| crate::Error::DeviceNotFound {
                source: Box::new(err),
            })?;

        Ok(WindowsConnection::new(mac_address.as_windows_u64())
            .await?
            .map(Arc::new))
    }
}
