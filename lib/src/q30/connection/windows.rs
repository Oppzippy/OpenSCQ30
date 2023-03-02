mod windows_connection;
mod windows_connection_descriptor;
mod windows_connection_registry;
mod windows_error;

pub use windows_connection::*;
pub use windows_connection_descriptor::*;
pub use windows_connection_registry::*;
pub use windows_error::*;

pub(crate) async fn new_connection_registry() -> crate::Result<WindowsConnectionRegistry> {
    WindowsConnectionRegistry::new().await
}
