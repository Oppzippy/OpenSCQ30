macro_rules! soundcore_device {
    (
        $state: ty,
        async |$packet_io_controller:ident| $fetch_state:block,
        async |$builder:ident| $block:block,
        $demo_packets:expr$(,)?
    ) => {
        soundcore_device! {
            $state,
            async |$packet_io_controller| $fetch_state,
            async |$builder| $block,
            $demo_packets,
            Default::default(),
        }
    };
    (
        $state: ty,
        async |$packet_io_controller:ident| $fetch_state:block,
        async |$builder:ident| $block:block,
        $demo_packets:expr,
        $config:expr$(,)?
    ) => {
        pub fn device_registry<B>(
            backend: B,
            database: std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::devices::DeviceModel,
        ) -> $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry<B, $state>
        where
            B: $crate::api::connection::RfcommBackend + Send + Sync + 'static,
        {
            $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry::new(
                backend,
                database,
                device_model,
                Box::new(|$packet_io_controller| Box::pin(async move { $fetch_state })),
                $config,
            )
        }

        pub fn demo_device_registry(
            database: std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::devices::DeviceModel,
        ) -> $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry<
            $crate::devices::soundcore::common::demo::DemoConnectionRegistry,
            $state,
        > {
            $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry::new(
                $crate::devices::soundcore::common::demo::DemoConnectionRegistry::new(
                    device_model,
                    $demo_packets,
                    $config,
                ),
                database,
                device_model,
                Box::new(|$packet_io_controller| Box::pin(async move { $fetch_state })),
                $config,
            )
        }

        impl<B> $crate::devices::soundcore::common::device::BuildDevice<B::ConnectionType, $state>
            for $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry<B, $state>
        where
            B: $crate::api::connection::RfcommBackend + Send + Sync + 'static,
        {
            async fn build_device(
                $builder: &mut $crate::devices::soundcore::common::device::SoundcoreDeviceBuilder<
                    B::ConnectionType,
                    $state,
                >,
            ) {
                $block
            }
        }
    };
}

pub(crate) use soundcore_device;
