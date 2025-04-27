mod rfcomm;
mod utils;

use crate::{api::connection, connection_backend::ConnectionBackends};

#[derive(Default)]
pub struct PlatformConnectionBackends {}

impl ConnectionBackends for PlatformConnectionBackends {
    type Rfcomm = rfcomm::WindowsRfcommBackend;

    async fn rfcomm(&self) -> connection::Result<Self::Rfcomm> {
        Ok(rfcomm::WindowsRfcommBackend::default())
    }
}
