// --- clippy::type_complexity ---
// Until this is resolved, there's not really a good workaround for having big types
// https://github.com/rust-lang/rust/issues/8995
// --- async_fn_in_trait ---
// This crate should not be used outside of this git repository, so breaking api changes are fine.
#![allow(clippy::type_complexity, async_fn_in_trait)]
pub mod api;
mod connection_backend;
pub mod demo;
pub mod device_profile;
pub mod device_utils;
pub mod devices;
mod error;
pub mod i18n;
pub(crate) mod macros;
pub mod soundcore_device;
pub mod storage;
pub mod stub;

pub use error::*;
