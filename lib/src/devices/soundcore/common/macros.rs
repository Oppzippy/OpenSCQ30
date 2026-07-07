/// Implements `device_registry` and `demo_device_registry` functions.
///
/// Parameters:
/// - state: type that holds state for the device
/// - initializer: async function that requests data needed to populate state and returns the current state
/// - builder: adds modules for the device's features
/// - demo_packets: HashMap<packet::Command, packet::Inbound> that will be referenced in `demo_device_registry` to respond to outbound packets
/// - config (optional): SoundcoreDeviceConfig for specifying other options, such as whether packets end with checksums
///
/// For examples, see any file in lib/src/device/soundcore/a*.rs
///
/// ```
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
        pub fn device_registry(
            backend: std::sync::Arc<dyn $crate::api::connection::RfcommBackend + Send + Sync>,
            database: std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::devices::DeviceModel,
        ) -> $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry<$state> {
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
        ) -> $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry<$state> {
            $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry::new(
                std::sync::Arc::new(
                    $crate::devices::soundcore::common::demo::DemoConnectionRegistry::new(
                        device_model,
                        $demo_packets,
                        $config,
                    ),
                ),
                database,
                device_model,
                Box::new(|$packet_io_controller| Box::pin(async move { $fetch_state })),
                $config,
            )
        }

        impl $crate::devices::soundcore::common::device::BuildDevice<$state>
            for $crate::devices::soundcore::common::device::SoundcoreDeviceRegistry<$state>
        {
            async fn build_device(
                $builder: &mut $crate::devices::soundcore::common::device::SoundcoreDeviceBuilder<
                    $state,
                >,
            ) {
                $block
            }
        }
    };
}

pub(crate) use soundcore_device;

/// Generates derives necessary for use with Setting::select_from_enum_all_variants as well as
/// take, bytes, and byte functions.
macro_rules! sound_mode_enum {
    (pub enum $enum_name:ident {
        $($variant:ident = $id:expr),+ $(,)?
    }) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            Ord,
            PartialOrd,
            Default,
            ::strum::FromRepr,
            ::strum::IntoStaticStr,
            ::strum::EnumString,
            ::strum::EnumIter,
            ::strum::VariantArray,
            ::openscq30_i18n_macros::Translate,
        )]
        #[repr(u8)]
        pub enum $enum_name {
            #[default]
            $($variant = $id),+
        }

        impl $enum_name {
            pub fn take<'a, E: ::nom::error::ParseError<&'a [u8]> + ::nom::error::ContextError<&'a [u8]>>(
                input: &'a [u8],
            ) -> ::nom::IResult<&'a [u8], Self, E> {
                use ::nom::Parser;
                ::nom::combinator::map(le_u8, |i| Self::from_repr(i).unwrap_or_default()).parse_complete(input)
            }

            pub fn bytes(&self) -> impl Iterator<Item = u8> {
                ::std::iter::once(*self as u8)
            }

            pub fn byte(&self) -> u8 {
                *self as u8
            }
        }
    };
}

pub(crate) use sound_mode_enum;
