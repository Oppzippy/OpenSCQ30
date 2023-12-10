#[cfg(not(target_arch = "wasm32"))]
mod futures_tokio;
#[cfg(feature = "wasm")]
mod futures_wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use futures_tokio::*;
#[cfg(feature = "wasm")]
pub use futures_wasm::*;

use futures::Future;
use std::time::Duration;

pub trait JoinHandle {
    fn abort(&self);
}

pub trait Futures {
    type JoinHandleType: JoinHandle;

    fn spawn<F, R>(future: F) -> Self::JoinHandleType
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static;
    fn spawn_local(future: impl Future + 'static) -> Self::JoinHandleType;
    async fn sleep(duration: Duration);
}
