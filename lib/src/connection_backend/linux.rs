mod rfcomm;

use crate::{api::connection, connection_backend::ConnectionBackends};

#[derive(Default)]
pub struct PlatformConnectionBackends {}

impl ConnectionBackends for PlatformConnectionBackends {
    type Rfcomm = rfcomm::BluerRfcommBackend;

    async fn rfcomm(&self) -> connection::Result<Self::Rfcomm> {
        rfcomm::BluerRfcommBackend::new().await
    }
}
