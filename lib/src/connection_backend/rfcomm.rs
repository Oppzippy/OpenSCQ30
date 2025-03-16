cfg_if::cfg_if! {
    if #[cfg(test)] {
        mod mock;
        pub use mock::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod bluer;
        pub use bluer::*;
    }
}
