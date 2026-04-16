use crate::api::connection::{self, RfcommBackend};

cfg_select! {
    target_os = "linux" => {
        mod linux;

    }
    target_os = "windows" => {
        mod windows;
    }
    _ => {
        mod none;
    }
}
#[cfg(test)]
pub(crate) mod mock;

/// Groups together platform specific implementations of various means of connecting to devices.
pub trait ConnectionBackends {
    type Rfcomm: RfcommBackend + Send + Sync;

    fn rfcomm(&self) -> impl Future<Output = connection::Result<Self::Rfcomm>> + Send;
}

pub fn default_backends() -> Option<impl ConnectionBackends> {
    cfg_select! {
        target_os = "linux" => {
            Some(linux::PlatformConnectionBackends::default())
        }
        target_os = "windows" => {
            Some(windows::PlatformConnectionBackends::default())
        }
        _ => {
            None::<none::NoneConnectionBackends>
        }
    }
}
