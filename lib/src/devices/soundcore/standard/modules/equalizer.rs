use std::sync::Arc;

use setting_handler::EqualizerSettingHandler;
use state_modifier::{
    EqualizerStateModifier, EqualizerStateModifierOptions, EqualizerWithBasicHearIdStateModifier,
    EqualizerWithCustomHearIdStateModifier,
};
use strum::{EnumIter, EnumString, IntoStaticStr};
use tokio::sync::watch;

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::{
        DeviceModel,
        soundcore::standard::{
            modules::equalizer::{
                custom_equalizer_profile_store::CustomEqualizerProfileStore,
                import_export_setting_handler::ImportExportSettingHandler,
            },
            packet::packet_io_controller::PacketIOController,
            structures::{AgeRange, BasicHearId, CustomHearId, EqualizerConfiguration, Gender},
        },
    },
    storage::OpenSCQ30Database,
};

use super::ModuleCollection;

mod custom_equalizer_profile_store;
mod import_export_setting_handler;
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
            EqualizerSetting::PresetProfile => Self::PresetEqualizerProfile,
            EqualizerSetting::CustomProfile => Self::CustomEqualizerProfile,
            EqualizerSetting::VolumeAdjustments => Self::VolumeAdjustments,
        }
    }
}

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum ImportExportSetting {
    ImportCustomEqualizerProfiles,
    ExportCustomEqualizerProfiles,
    ExportCustomEqualizerProfilesOutput,
}

impl From<ImportExportSetting> for SettingId {
    fn from(value: ImportExportSetting) -> Self {
        match value {
            ImportExportSetting::ImportCustomEqualizerProfiles => {
                Self::ImportCustomEqualizerProfiles
            }
            ImportExportSetting::ExportCustomEqualizerProfiles => {
                Self::ExportCustomEqualizerProfiles
            }
            ImportExportSetting::ExportCustomEqualizerProfilesOutput => {
                Self::ExportCustomEqualizerProfilesOutput
            }
        }
    }
}

impl TryFrom<&SettingId> for ImportExportSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::ImportCustomEqualizerProfiles => Ok(Self::ImportCustomEqualizerProfiles),
            SettingId::ExportCustomEqualizerProfiles => Ok(Self::ExportCustomEqualizerProfiles),
            SettingId::ExportCustomEqualizerProfilesOutput => {
                Ok(Self::ExportCustomEqualizerProfilesOutput)
            }
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T> {
    pub async fn add_equalizer<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
    {
        self.add_equalizer_setting_handlers(database, device_model, change_notify)
            .await;
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
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
    {
        self.add_equalizer_setting_handlers(database, device_model, change_notify)
            .await;
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
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
        T: AsRef<BasicHearId<CHANNELS, BANDS>> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.add_equalizer_setting_handlers(database, device_model, change_notify)
            .await;
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
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
        T: AsRef<CustomHearId<CHANNELS, BANDS>> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.add_equalizer_setting_handlers(database, device_model, change_notify)
            .await;
        self.state_modifiers
            .push(Box::new(EqualizerWithCustomHearIdStateModifier::new(
                packet_io,
            )));
    }

    async fn add_equalizer_setting_handlers<const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        T: AsMut<EqualizerConfiguration<CHANNELS, BANDS>>
            + AsRef<EqualizerConfiguration<CHANNELS, BANDS>>
            + Clone
            + Send
            + Sync,
    {
        let profile_store = Arc::new(
            CustomEqualizerProfileStore::new(database, device_model, change_notify.to_owned())
                .await,
        );
        self.setting_manager.add_handler(
            CategoryId::Equalizer,
            EqualizerSettingHandler::<CHANNELS, BANDS>::new(profile_store.to_owned()).await,
        );
        self.setting_manager.add_handler(
            CategoryId::EqualizerImportExport,
            ImportExportSettingHandler::new(profile_store, change_notify).await,
        );
    }
}
