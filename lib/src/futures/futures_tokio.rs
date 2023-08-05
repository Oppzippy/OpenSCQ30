use std::time::Duration;

use futures::Future;

use super::JoinHandle;

#[derive(Debug)]
pub struct TokioJoinHandle(tokio::task::JoinHandle<()>);

impl JoinHandle for TokioJoinHandle {
    fn abort(&self) {
        self.0.abort()
    }
}

// tokio's spawn_local returns a JoinHandle, but wasm_bindgen_futures does not, so we can't return
// one here.
pub fn spawn_local(future: impl Future + 'static) -> TokioJoinHandle {
    let join_handle = tokio::task::spawn_local(async move {
        future.await;
    });
    TokioJoinHandle(join_handle)
}

pub fn sleep(duration: Duration) -> impl Future + 'static {
    tokio::time::sleep(duration)
}
