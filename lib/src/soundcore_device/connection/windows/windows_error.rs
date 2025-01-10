use std::panic::Location;

impl From<windows::core::Error> for crate::Error {
    #[track_caller]
    fn from(err: windows::core::Error) -> Self {
        match err.code().0 as u32 {
            // The object has been closed.
            0x80000013 => crate::Error::NotConnected {
                source: Some(Box::new(err)),
            },
            _ => crate::Error::Other {
                source: Box::new(err),
                location: Location::caller(),
            },
        }
    }
}
