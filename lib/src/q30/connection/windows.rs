mod windows_connection;
mod windows_connection_registry;
mod windows_error;
mod windows_mac_addess;

pub use windows_connection::*;
pub use windows_connection_registry::*;
pub use windows_error::*;
pub(crate) use windows_mac_addess::*;

pub(crate) async fn new_connection_registry() -> crate::Result<WindowsConnectionRegistry> {
    WindowsConnectionRegistry::new().await
}
