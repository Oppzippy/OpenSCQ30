impl From<btleplug::Error> for crate::Error {
    fn from(err: btleplug::Error) -> Self {
        match err {
            btleplug::Error::DeviceNotFound => crate::Error::DeviceNotFound {
                source: Box::new(err),
            },
            btleplug::Error::NotConnected => crate::Error::NotConnected {
                source: Box::new(err),
            },
            _ => crate::Error::Other {
                source: Box::new(err),
            },
        }
    }
}
