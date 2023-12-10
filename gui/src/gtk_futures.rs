use std::time::Duration;

use futures::Future;
use gtk::glib::{self, MainContext};
use openscq30_lib::futures::{Futures, JoinHandle};

pub struct GtkFutures {}

impl Futures for GtkFutures {
    type JoinHandleType = GtkJoinHandle;

    fn spawn<F, R>(future: F) -> Self::JoinHandleType
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let join_handle = MainContext::default().spawn(async {
            future.await;
        });
        GtkJoinHandle(join_handle)
    }

    fn spawn_local(future: impl Future + 'static) -> Self::JoinHandleType {
        let join_handle = MainContext::default().spawn_local(async {
            future.await;
        });
        GtkJoinHandle(join_handle)
    }

    async fn sleep(duration: Duration) {
        glib::timeout_future(duration).await;
    }
}

pub struct GtkJoinHandle(glib::JoinHandle<()>);

impl JoinHandle for GtkJoinHandle {
    fn abort(&self) {
        self.0.abort()
    }
}
