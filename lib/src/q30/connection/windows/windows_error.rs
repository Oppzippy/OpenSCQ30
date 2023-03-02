impl From<windows::core::Error> for crate::Error {
    fn from(err: windows::core::Error) -> Self {
        crate::Error::Other {
            source: Box::new(err),
        }
    }
}
