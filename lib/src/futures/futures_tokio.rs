use std::time::Duration;

use futures::Future;

use super::{Futures, JoinHandle, MaybeSend};

pub struct TokioFutures;

impl Futures for TokioFutures {
    type JoinHandleType = TokioJoinHandle;

    fn spawn<F, R>(future: F) -> Self::JoinHandleType
    where
        F: Future<Output = R> + MaybeSend + 'static,
        R: MaybeSend + 'static,
    {
        let join_handle = tokio::task::spawn(async move {
            future.await;
        });
        TokioJoinHandle(join_handle)
    }

    async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}

#[derive(Debug)]
pub struct TokioJoinHandle(tokio::task::JoinHandle<()>);

impl JoinHandle for TokioJoinHandle {
    fn abort(&self) {
        self.0.abort()
    }
}
