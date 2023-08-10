use std::{collections::HashSet, rc::Rc, time::Duration};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tracing::instrument;
use windows::{
    core::HSTRING,
    Devices::{
        Bluetooth::{self, BluetoothConnectionStatus, BluetoothDevice},
        Enumeration::{DeviceInformation, DeviceInformationKind},
    },
    Foundation::{Collections::IVectorView, TypedEventHandler},
};

use crate::api::connection::{ConnectionRegistry, GenericConnectionDescriptor};

use super::{WindowsConnection, WindowsMacAddress};

pub struct WindowsConnectionRegistry {}

impl WindowsConnectionRegistry {
    pub async fn new() -> crate::Result<Self> {
        Ok(Self {})
    }

    /// We can obtain a BluetoothDevice now with the device id, but Windows may not know that it is a
    /// bluetooth LE device. We need to scan for devices in order to get BluetoothLEDevice not not be
    /// HRESULT 0.
    fn scan_for_le_devices() -> crate::Result<()> {
        let props = vec![
            // I got these from the example in Microsoft's documentation
            // We might only need "System.Devices.Aep.Bluetooth.Le.IsConnectable" since we already know the
            // address and whether or not it is connected.
            HSTRING::from("System.Devices.Aep.DeviceAddress"),
            HSTRING::from("System.Devices.Aep.IsConnected"),
            HSTRING::from("System.Devices.Aep.Bluetooth.Le.IsConnectable"),
        ];
        let iterable = IVectorView::<HSTRING>::try_from(props)?;
        let watcher = DeviceInformation::CreateWatcherWithKindAqsFilterAndAdditionalProperties(
            // Bluetooth LE protocol ID: https://learn.microsoft.com/en-us/windows/uwp/devices-sensors/aep-service-class-ids#bluetooth-and-bluetooth-le-services
            &HSTRING::from(
                "(System.Devices.Aep.ProtocolId:=\"{bb7bb05e-5972-42b5-94fc-76eaa7084d49}\")",
            ),
            &iterable,
            DeviceInformationKind::AssociationEndpoint,
        )?;
        // We don't actually need any event handlers, since we're just trying to get Windows the information, not
        // ourselves. It errors though without an event handler, so we might as well log.
        watcher.Added(&TypedEventHandler::new(
            |_, device_information: &Option<DeviceInformation>| {
                if let Some(device_information) = device_information {
                    tracing::trace!(
                        "added device: {}",
                        device_information.Name().unwrap_or(HSTRING::from("?")),
                    );
                }
                Ok(())
            },
        ))?;
        watcher.Start()?;
        // The Soundcore Q30 seems to advertise around every .3s
        // Wait some extra time to be safe
        std::thread::sleep(Duration::from_secs(1));
        watcher.Stop()?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl ConnectionRegistry for WindowsConnectionRegistry {
    type ConnectionType = WindowsConnection;
    type DescriptorType = GenericConnectionDescriptor;

    #[instrument(level = "trace", skip(self))]
    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>> {
        Self::scan_for_le_devices()?;
        let devices = DeviceInformation::FindAllAsyncAqsFilter(
            &Bluetooth::BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
                BluetoothConnectionStatus::Connected,
            )?,
        )?
        .await?;

        let mut descriptors = HashSet::new();
        for device in devices {
            let id = device.Id()?;

            let bluetooth_device = BluetoothDevice::FromIdAsync(&id)?.await?;
            let mac_address = MacAddr6::from_windows_u64(bluetooth_device.BluetoothAddress()?);

            let descriptor =
                GenericConnectionDescriptor::new(bluetooth_device.Name()?.to_string(), mac_address);
            descriptors.insert(descriptor);
        }
        Ok(descriptors)
    }

    #[instrument(level = "trace", skip(self))]
    async fn connection(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Option<Rc<Self::ConnectionType>>> {
        Ok(WindowsConnection::new(mac_address).await?.map(Rc::new))
    }
}
