macro_rules! soundcore_device {
    ($state: ty, $state_update_packet: ty, async |$builder:ident| $block:block) => {
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
            B: $crate::api::connection::RfcommBackend + Send + Sync,
        {
            $crate::devices::soundcore::standard::device::SoundcoreDeviceRegistry::new(
                backend,
                database,
                device_model,
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
            use $crate::devices::soundcore::standard::packets::outbound::OutboundPacketBytesExt;
            $crate::devices::soundcore::standard::device::SoundcoreDeviceRegistry::new(
                $crate::devices::soundcore::standard::demo::DemoConnectionRegistry::new(
                    device_model.to_string(),
                    <$state_update_packet>::default().bytes(),
                ),
                database,
                device_model,
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
            B: $crate::api::connection::RfcommBackend + Send + Sync,
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

pub(crate) use soundcore_device;
