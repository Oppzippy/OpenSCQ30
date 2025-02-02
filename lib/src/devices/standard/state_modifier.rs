use async_trait::async_trait;
use tokio::sync::watch;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait StateModifier<T> {
    async fn move_to_state(&self, state: &watch::Sender<T>, target_state: &T) -> crate::Result<()>;
}
