#[cfg(not(target_arch = "wasm32"))]
mod futures_tokio;
#[cfg(target_arch = "wasm32")]
mod futures_wasm;

use std::time::Duration;

use futures::Future;

pub trait JoinHandle {
    fn abort(&self);
}

// tokio's spawn_local returns a JoinHandle, but wasm_bindgen_futures does not, so we can't return
// one here.
pub fn spawn(future: impl Future + Send + 'static) -> impl JoinHandle {
    #[cfg(not(target_arch = "wasm32"))]
    {
        futures_tokio::spawn(future)
    }
    #[cfg(target_arch = "wasm32")]
    {
        futures_wasm::spawn_local(future)
    }
}

pub fn sleep(duration: Duration) -> impl Future + 'static {
    #[cfg(not(target_arch = "wasm32"))]
    {
        futures_tokio::sleep(duration)
    }
    #[cfg(target_arch = "wasm32")]
    {
        futures_wasm::sleep(duration)
    }
}
