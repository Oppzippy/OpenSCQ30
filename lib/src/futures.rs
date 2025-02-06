#[cfg(not(target_arch = "wasm32"))]
mod futures_tokio;

#[cfg(not(target_arch = "wasm32"))]
pub use futures_tokio::*;
#[cfg(target_arch = "wasm32")]
pub use futures_wasm::*;

use futures::Future;
use std::time::Duration;

pub trait JoinHandle {
    fn abort(&self);
}

pub trait Futures {
    type JoinHandleType: JoinHandle + MaybeSend + MaybeSync;

    fn spawn<F, R>(future: F) -> Self::JoinHandleType
    where
        F: Future<Output = R> + MaybeSend + 'static,
        R: MaybeSend + 'static;
    fn sleep(duration: Duration) -> impl Future<Output = ()> + MaybeSend;
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    /// An extension trait that enforces `Send` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSend: Send {}

    impl<T> MaybeSend for T where T: Send {}

    /// An extension trait that enforces `Sync` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSync: Sync {}

    impl<T> MaybeSync for T where T: Sync {}
}

#[cfg(target_arch = "wasm32")]
mod platform {
    /// An extension trait that enforces `Send` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSend {}

    impl<T> MaybeSend for T {}

    /// An extension trait that enforces `Sync` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSync {}

    impl<T> MaybeSync for T {}
}

pub use platform::{MaybeSend, MaybeSync};
