impl From<bluer::Error> for crate::Error {
    fn from(error: bluer::Error) -> Self {
        crate::Error::Other {
            source: Box::new(error),
        }
    }
}
