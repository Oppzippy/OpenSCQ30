#[macro_export]
/// example: async_runtime_bridge!(self.tokio_runtime, self.soundcore_device_registry.devices().await)
macro_rules! async_runtime_bridge {
    ( $tokio_runtime: expr, $expr: expr $(,)?) => {{
        let (sender, receiver) = futures::channel::oneshot::channel();

        $tokio_runtime.spawn(async move {
            sender.send($expr).unwrap();
        });

        receiver.await.unwrap()
    }};
}
