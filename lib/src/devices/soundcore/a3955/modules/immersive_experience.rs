mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::ImmersiveExperienceSettingHandler;
use state_modifier::ImmersiveExperienceStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3955::structures::ImmersiveExperience,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum ImmersiveExperienceSetting {
        ImmersiveExperience,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<ImmersiveExperience> + Clone + Send + Sync,
{
    pub fn add_a3955_immersive_experience<ConnectionT>(
        &mut self,
        packet_io: Arc<PacketIOController<ConnectionT>>,
    ) where
        ConnectionT: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            ImmersiveExperienceSettingHandler::default(),
        );
        self.state_modifiers
            .push(Box::new(ImmersiveExperienceStateModifier::new(packet_io)));
    }
}
