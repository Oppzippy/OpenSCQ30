macro_rules! soundcore_device {
    ($name:ident with $state:ident initialized by $state_update_packet:ident => {
        $($module_collection_additions_fn:ident $module_colection_additions:tt;)*
    }) => {
        pub fn device_registry<B: $crate::api::connection::RfcommBackend>(
            backend: B,
            database: ::std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::soundcore_device::device_model::DeviceModel,
        ) -> $crate::devices::standard::device::SoundcoreDeviceRegistry<B, $name<B::ConnectionType>> {
            $crate::devices::standard::device::SoundcoreDeviceRegistry::new(backend, database, device_model)
        }

        pub fn demo_device_registry(
            database: ::std::sync::Arc<$crate::storage::OpenSCQ30Database>,
            device_model: $crate::soundcore_device::device_model::DeviceModel,
        ) -> $crate::devices::standard::device::SoundcoreDeviceRegistry<
            $crate::devices::standard::demo::DemoConnectionRegistry,
            $name<<$crate::devices::standard::demo::DemoConnectionRegistry as $crate::api::connection::RfcommBackend>::ConnectionType>,
        > {
            use crate::devices::standard::packets::outbound::OutboundPacketBytesExt;
            $crate::devices::standard::device::SoundcoreDeviceRegistry::new(
                $crate::devices::standard::demo::DemoConnectionRegistry::new(
                    device_model.to_string(),
                    $state_update_packet::default().bytes(),
                ),
                database,
                device_model,
            )
        }

        pub struct $name<ConnectionType: $crate::api::connection::RfcommConnection + Send + Sync> {
            device_model: $crate::soundcore_device::device_model::DeviceModel,
            state_sender: ::tokio::sync::watch::Sender<$state>,
            module_collection: ::std::sync::Arc<$crate::devices::standard::modules::ModuleCollection<$state>>,
            _packet_io_controller: ::std::sync::Arc<
                $crate::soundcore_device::device::packet_io_controller::PacketIOController<
                    ConnectionType,
                >,
            >,
        }

        impl<ConnectionType> $crate::devices::standard::device::SoundcoreDevice<ConnectionType> for $name<ConnectionType>
        where
            ConnectionType: $crate::api::connection::RfcommConnection + 'static + Send + Sync,
        {
            async fn new(
                database: ::std::sync::Arc<$crate::storage::OpenSCQ30Database>,
                connection: ConnectionType,
                device_model: $crate::soundcore_device::device_model::DeviceModel,
            ) -> crate::Result<Self> {
                use crate::devices::standard::packets::inbound::TryIntoInboundPacket;
                use crate::devices::standard::modules::ModuleCollectionSpawnPacketHandlerExt;

                let (packet_io_controller, packet_receiver) =
                    $crate::soundcore_device::device::packet_io_controller::PacketIOController::<ConnectionType>::new(::std::sync::Arc::new(connection)).await?;
                let packet_io_controller = ::std::sync::Arc::new(packet_io_controller);
                let state_update_packet: $state_update_packet = packet_io_controller
                    .send(&$crate::devices::standard::packets::outbound::RequestStatePacket::new().into())
                    .await?
                    .try_into_inbound_packet()?;
                let (state_sender, _) = ::tokio::sync::watch::channel::<$state>(state_update_packet.into());

                let mut module_collection = $crate::devices::standard::modules::ModuleCollection::<$state>::default();
                $(
                    soundcore_device!(
                        module_collection,
                        packet_io_controller,
                        database,
                        device_model,
                        $module_collection_additions_fn $module_colection_additions);
                )*

                let module_collection = ::std::sync::Arc::new(module_collection);
                module_collection.spawn_packet_handler(state_sender.clone(), packet_receiver);

                Ok(Self {
                    device_model,
                    state_sender,
                    _packet_io_controller: packet_io_controller,
                    module_collection,
                })
            }
        }

        $crate::devices::standard::macros::impl_soundcore_device!(
            $name,
            model = device_model,
            module_collection = module_collection,
            state_sender = state_sender
        );
    };
    ($module_collection:ident, $packet_io_controller:ident, $database:ident, $device_model:ident, state_update ()) => {
        $module_collection.add_state_update();
    };
    ($module_collection:ident, $packet_io_controller:ident, $database:ident, $device_model:ident, sound_modes $available_sound_modes:expr) => {
        $module_collection.add_sound_modes($packet_io_controller.clone(), $available_sound_modes);
    };
    ($module_collection:ident, $packet_io_controller:ident, $database:ident, $device_model:ident, equalizer (mono)) => {
        $module_collection.add_equalizer(
            $packet_io_controller.clone(),
            $database.clone(),
            $device_model,
            false,
        ).await;
    };
    ($module_collection:ident, $packet_io_controller:ident, $database:ident, $device_model:ident, equalizer (stereo)) => {
        $module_collection.add_equalizer(
            $packet_io_controller.clone(),
            $database.clone(),
            $device_model,
            true,
        ).await;
    };
    ($module_collection:ident, $packet_io_controller:ident, $database:ident, $device_model:ident, equalizer_with_basic_hear_id ()) => {
        $module_collection.add_equalizer_with_basic_hear_id(
            $packet_io_controller.clone(),
            $database.clone(),
            $device_model,
        ).await;
    };
    ($module_collection:ident, $packet_io_controller:ident, $database:ident, $device_model:ident, equalizer_with_custom_hear_id ()) => {
        $module_collection.add_equalizer_with_custom_hear_id(
            $packet_io_controller.clone(),
            $database.clone(),
            $device_model,
        ).await;
    };
    ($module_collection:ident, $packet_io_controller:ident, $database:ident, $device_model:ident, button_configuration ()) => {
        $module_collection.add_button_configuration($packet_io_controller.clone());
    };
}
pub(crate) use soundcore_device;

macro_rules! impl_soundcore_device {
    ($struct:ident, model = $model:ident, module_collection = $module_collection:ident, state_sender = $state_sender:ident) => {
        #[::async_trait::async_trait]
        impl<ConnectionType> $crate::api::device::OpenSCQ30Device for $struct<ConnectionType>
        where
            ConnectionType: $crate::api::connection::RfcommConnection + 'static + Send + Sync,
        {
            fn model(&self) -> $crate::soundcore_device::device_model::DeviceModel {
                self.$model
            }

            fn categories(&self) -> ::std::vec::Vec<$crate::api::settings::CategoryId> {
                self.$module_collection
                    .setting_manager
                    .categories()
                    .to_vec()
            }

            fn settings_in_category(
                &self,
                category_id: &$crate::api::settings::CategoryId,
            ) -> ::std::vec::Vec<$crate::api::settings::SettingId> {
                self.$module_collection
                    .setting_manager
                    .category(category_id)
            }

            fn setting(
                &self,
                setting_id: &$crate::api::settings::SettingId,
            ) -> ::std::option::Option<$crate::api::settings::Setting> {
                let state = self.$state_sender.borrow().to_owned();
                self.$module_collection
                    .setting_manager
                    .get(&state, setting_id)
            }

            async fn set_setting_values(
                &self,
                setting_values: Vec<(
                    $crate::api::settings::SettingId,
                    $crate::api::settings::Value,
                )>,
            ) -> $crate::Result<()> {
                let mut target_state = self.$state_sender.borrow().clone();
                for (setting_id, value) in setting_values {
                    self.$module_collection
                        .setting_manager
                        .set(&mut target_state, &setting_id, value)
                        .await
                        .unwrap()?;
                }
                for modifier in &self.$module_collection.state_modifiers {
                    modifier
                        .move_to_state(&self.$state_sender, &target_state)
                        .await?;
                }
                Ok(())
            }
        }
    };
}
pub(crate) use impl_soundcore_device;
