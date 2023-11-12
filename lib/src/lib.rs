// Until this is resolved, there's not really a good workaround for having big types
// https://github.com/rust-lang/rust/issues/8995
#![allow(clippy::type_complexity)]
pub mod api;
pub mod demo;
pub mod device_profiles;
pub mod device_utils;
pub mod devices;
mod error;
pub mod futures;
pub mod soundcore_device;
pub mod stub;

pub use error::*;
