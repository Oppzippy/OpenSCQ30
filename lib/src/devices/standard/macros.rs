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
