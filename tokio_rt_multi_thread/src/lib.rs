use tokio::runtime::Builder;

/**
 * Tokio fails to build for wasm if the "rt-multi-thread" feature is enabled. Cargo doesn't offer
 * a way to turn on/off features per build target, so instead it is separated out into another crate
 * that can be conditionally enabled.
 */
pub fn new_multi_thread() -> Builder {
    tokio::runtime::Builder::new_multi_thread()
}
