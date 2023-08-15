// Until this is resolved, there's not really a good workaround for having big types
// https://github.com/rust-lang/rust/issues/8995
#![allow(clippy::type_complexity)]
pub mod api;
pub mod demo;
pub mod device_utils;
mod error;
pub mod futures;
pub mod packets;
pub mod q30;
pub mod state;
pub mod stub;

pub use error::*;
