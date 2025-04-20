use crate::api::connection::{self, RfcommBackend};

pub mod rfcomm;

pub trait ConnectionBackends {
    type Rfcomm: RfcommBackend + Send + Sync;

    fn rfcomm(&self) -> impl Future<Output = connection::Result<Self::Rfcomm>> + Send;
}

pub fn default_backends() -> Option<impl ConnectionBackends> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            Some(PlatformConnectionBackends {})
        } else {
            None
        }
    }
}

#[cfg(target_os = "linux")]
struct PlatformConnectionBackends {}

#[cfg(target_os = "linux")]
impl ConnectionBackends for PlatformConnectionBackends {
    type Rfcomm = rfcomm::BluerRfcommBackend;

    async fn rfcomm(&self) -> connection::Result<Self::Rfcomm> {
        rfcomm::BluerRfcommBackend::new().await
    }
}
