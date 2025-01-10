use std::panic::Location;

impl From<bluer::Error> for crate::Error {
    #[track_caller]
    fn from(error: bluer::Error) -> Self {
        crate::Error::Other {
            source: Box::new(error),
            location: Location::caller(),
        }
    }
}
