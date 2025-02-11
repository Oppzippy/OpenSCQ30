use std::sync::Arc;

use setting_handler::EqualizerSettingHandler;
use state_modifier::EqualizerStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{connection::Connection, settings::CategoryId},
    devices::standard::structures::EqualizerConfiguration,
    futures::{Futures, MaybeSend, MaybeSync},
    soundcore_device::device::packet_io_controller::PacketIOController,
};

use super::ModuleCollection;

mod setting_handler;
mod state_modifier;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum EqualizerSetting {
    PresetProfile,
    CustomProfile,
    VolumeAdjustments,
}

pub trait AddEqualizerExt {
    fn add_equalizer<C, F>(&mut self, packet_io: Arc<PacketIOController<C, F>>, is_stereo: bool)
    where
        C: Connection + 'static + MaybeSend + MaybeSync,
        F: Futures + 'static + MaybeSend + MaybeSync;
}

impl<T> AddEqualizerExt for ModuleCollection<T>
where
    T: AsMut<EqualizerConfiguration>
        + AsRef<EqualizerConfiguration>
        + Clone
        + MaybeSend
        + MaybeSync,
{
    fn add_equalizer<C, F>(&mut self, packet_io: Arc<PacketIOController<C, F>>, is_stereo: bool)
    where
        C: Connection + 'static + MaybeSend + MaybeSync,
        F: Futures + 'static + MaybeSend + MaybeSync,
    {
        self.setting_manager
            .add_handler(CategoryId("equalizer"), EqualizerSettingHandler::default());
        self.state_modifiers
            .push(Box::new(EqualizerStateModifier::new(packet_io, is_stereo)));
    }
}
