macro_rules! soundcore_device {
    ($state: ty, $state_update_packet: ty, async |$builder:ident| $block:block) => {
        pub fn device_registry<B>(
            backend: B,
            database: std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::soundcore_device::device_model::DeviceModel,
        ) -> $crate::devices::standard::device::SoundcoreDeviceRegistry<
            B,
            $state,
            $state_update_packet,
        >
        where
            B: $crate::api::connection::RfcommBackend + Send + Sync,
        {
            $crate::devices::standard::device::SoundcoreDeviceRegistry::new(
                backend,
                database,
                device_model,
            )
        }

        pub fn demo_device_registry(
            database: std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::soundcore_device::device_model::DeviceModel,
        ) -> $crate::devices::standard::device::SoundcoreDeviceRegistry<
            $crate::devices::standard::demo::DemoConnectionRegistry,
            $state,
            $state_update_packet,
        > {
            use $crate::devices::standard::packets::outbound::OutboundPacketBytesExt;
            $crate::devices::standard::device::SoundcoreDeviceRegistry::new(
                $crate::devices::standard::demo::DemoConnectionRegistry::new(
                    device_model.to_string(),
                    <$state_update_packet>::default().bytes(),
                ),
                database,
                device_model,
            )
        }

        impl<B>
            $crate::devices::standard::device::BuildDevice<
                B::ConnectionType,
                $state,
                $state_update_packet,
            >
            for $crate::devices::standard::device::SoundcoreDeviceRegistry<
                B,
                $state,
                $state_update_packet,
            >
        where
            B: $crate::api::connection::RfcommBackend + Send + Sync,
        {
            async fn build_device(
                $builder: &mut $crate::devices::standard::device::SoundcoreDeviceBuilder<
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

pub(crate) use soundcore_device;
