use std::sync::Arc;

use setting_handler::EqualizerSettingHandler;
use state_modifier::{
    EqualizerStateModifier, EqualizerStateModifierOptions, EqualizerWithBasicHearIdStateModifier,
    EqualizerWithCustomHearIdStateModifier,
};
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::{
        DeviceModel,
        soundcore::standard::{
            packets::packet_io_controller::PacketIOController,
            structures::{AgeRange, BasicHearId, CustomHearId, EqualizerConfiguration, Gender},
        },
    },
    storage::OpenSCQ30Database,
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

impl TryFrom<&SettingId> for EqualizerSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::PresetEqualizerProfile => Ok(Self::PresetProfile),
            SettingId::CustomEqualizerProfile => Ok(Self::CustomProfile),
            SettingId::VolumeAdjustments => Ok(Self::VolumeAdjustments),
            _ => Err(()),
        }
    }
}

impl From<EqualizerSetting> for SettingId {
    fn from(setting: EqualizerSetting) -> Self {
        match setting {
            EqualizerSetting::PresetProfile => SettingId::PresetEqualizerProfile,
            EqualizerSetting::CustomProfile => SettingId::CustomEqualizerProfile,
            EqualizerSetting::VolumeAdjustments => SettingId::VolumeAdjustments,
        }
    }
}

impl<T> ModuleCollection<T> {
    pub async fn add_equalizer<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::Equalizer,
            EqualizerSettingHandler::new(database, device_model).await,
        );
        self.state_modifiers
            .push(Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: false },
            )));
    }

    pub async fn add_equalizer_with_drc<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::Equalizer,
            EqualizerSettingHandler::new(database, device_model).await,
        );
        self.state_modifiers
            .push(Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: true },
            )));
    }
    pub async fn add_equalizer_with_basic_hear_id<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
        T: AsRef<BasicHearId<CHANNELS, BANDS>> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.setting_manager.add_handler(
            CategoryId::Equalizer,
            EqualizerSettingHandler::<CHANNELS, BANDS>::new(database, device_model).await,
        );
        self.state_modifiers
            .push(Box::new(EqualizerWithBasicHearIdStateModifier::<
                Conn,
                CHANNELS,
                BANDS,
            >::new(packet_io)));
    }

    pub async fn add_equalizer_with_custom_hear_id<
        Conn,
        const CHANNELS: usize,
        const BANDS: usize,
    >(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
        T: AsRef<CustomHearId<CHANNELS, BANDS>> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.setting_manager.add_handler(
            CategoryId::Equalizer,
            EqualizerSettingHandler::new(database, device_model).await,
        );
        self.state_modifiers
            .push(Box::new(EqualizerWithCustomHearIdStateModifier::new(
                packet_io,
            )));
    }
}
