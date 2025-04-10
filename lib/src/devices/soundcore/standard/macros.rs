macro_rules! soundcore_device {
    (
        $state: ty,
        $state_update_packet: ty,
        async |$packet_io_controller:ident| $fetch_state:block,
        async |$builder:ident| $block:block,
        $demo_packets:expr$(,)?
    ) => {
        pub fn device_registry<B>(
            backend: B,
            database: std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::devices::DeviceModel,
        ) -> $crate::devices::soundcore::standard::device::SoundcoreDeviceRegistry<
            B,
            $state,
            $state_update_packet,
        >
        where
            B: $crate::api::connection::RfcommBackend + Send + Sync + 'static,
        {
            $crate::devices::soundcore::standard::device::SoundcoreDeviceRegistry::new(
                backend,
                database,
                device_model,
                Box::new(|$packet_io_controller| Box::pin(async move { $fetch_state })),
            )
        }

        pub fn demo_device_registry(
            database: std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::devices::DeviceModel,
        ) -> $crate::devices::soundcore::standard::device::SoundcoreDeviceRegistry<
            $crate::devices::soundcore::standard::demo::DemoConnectionRegistry,
            $state,
            $state_update_packet,
        > {
            $crate::devices::soundcore::standard::device::SoundcoreDeviceRegistry::new(
                $crate::devices::soundcore::standard::demo::DemoConnectionRegistry::new(
                    device_model.to_string(),
                    $demo_packets,
                ),
                database,
                device_model,
                Box::new(|$packet_io_controller| Box::pin(async move { $fetch_state })),
            )
        }

        impl<B>
            $crate::devices::soundcore::standard::device::BuildDevice<
                B::ConnectionType,
                $state,
                $state_update_packet,
            >
            for $crate::devices::soundcore::standard::device::SoundcoreDeviceRegistry<
                B,
                $state,
                $state_update_packet,
            >
        where
            B: $crate::api::connection::RfcommBackend + Send + Sync + 'static,
        {
            async fn build_device(
                $builder: &mut $crate::devices::soundcore::standard::device::SoundcoreDeviceBuilder<
                    B::ConnectionType,
                    $state,
                    $state_update_packet,
                >,
            ) {
                $block
            }
        }
    };
}

macro_rules! default_demo_responses {
    ($($command:expr => $packet_type:ty),* $(,)?) => {{
        ::std::collections::HashMap::from([
            $((
                $command,
                <$packet_type as $crate::devices::soundcore::standard::packets::outbound::OutboundPacketBytesExt>::bytes(<$packet_type>::default()),
            ),)*
        ])
    }};
}

pub(crate) use default_demo_responses;
pub(crate) use soundcore_device;
