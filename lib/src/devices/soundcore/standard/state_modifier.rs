use async_trait::async_trait;
use tokio::sync::watch;

use crate::api::device;

#[async_trait]
pub trait StateModifier<T> {
    async fn move_to_state(&self, state: &watch::Sender<T>, target_state: &T)
    -> device::Result<()>;
}
