mod rfcomm;

use crate::{api::connection, connection_backend::ConnectionBackends};

#[derive(Default)]
pub struct PlatformConnectionBackends;

impl ConnectionBackends for PlatformConnectionBackends {
    type Rfcomm = rfcomm::MacosRfcommBackend;

    async fn rfcomm(&self) -> connection::Result<Self::Rfcomm> {
        Ok(rfcomm::MacosRfcommBackend)
    }
}
