use cfg_if::cfg_if;

use crate::api::connection::{self, RfcommBackend};

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
    } else {
        mod none;
    }

}
#[cfg(test)]
pub mod mock;

pub trait ConnectionBackends {
    type Rfcomm: RfcommBackend + Send + Sync;

    fn rfcomm(&self) -> impl Future<Output = connection::Result<Self::Rfcomm>> + Send;
}

pub fn default_backends() -> Option<impl ConnectionBackends> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            Some(linux::PlatformConnectionBackends::default())
        } else {
            None::<none::NoneConnectionBackends>
        }
    }
}
