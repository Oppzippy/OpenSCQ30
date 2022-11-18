use crate::soundcore_bluetooth::traits::soundcore_device_connection_error::SoundcoreDeviceConnectionError;

impl From<btleplug::Error> for SoundcoreDeviceConnectionError {
    fn from(err: btleplug::Error) -> Self {
        match err {
            btleplug::Error::DeviceNotFound => SoundcoreDeviceConnectionError::DeviceNotFound {
                source: Box::new(err),
            },
            btleplug::Error::NotConnected => SoundcoreDeviceConnectionError::NotConnected {
                source: Box::new(err),
            },
            _ => SoundcoreDeviceConnectionError::Other {
                source: Box::new(err),
            },
        }
    }
}
