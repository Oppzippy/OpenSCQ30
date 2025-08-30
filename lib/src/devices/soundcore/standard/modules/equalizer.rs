use std::sync::Arc;

use openscq30_lib_has::Has;
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
    macros::enum_subset,
    storage::OpenSCQ30Database,
};

use super::ModuleCollection;

mod custom_equalizer_profile_store;
mod import_export_setting_handler;
mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum EqualizerSetting {
        PresetEqualizerProfile,
        CustomEqualizerProfile,
        VolumeAdjustments,
    }
);

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum ImportExportSetting {
        ImportCustomEqualizerProfiles,
        ExportCustomEqualizerProfiles,
        ExportCustomEqualizerProfilesOutput,
    }
);

impl<T> ModuleCollection<T> {
    pub async fn add_equalizer<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: Has<EqualizerConfiguration<CHANNELS, BANDS>> + Clone + Send + Sync,
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
        T: Has<EqualizerConfiguration<CHANNELS, BANDS>> + Clone + Send + Sync,
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
        T: Has<EqualizerConfiguration<CHANNELS, BANDS>>
            + Has<BasicHearId<CHANNELS, BANDS>>
            + Has<Gender>
            + Has<AgeRange>
            + Clone
            + Send
            + Sync,
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
        T: Has<EqualizerConfiguration<CHANNELS, BANDS>>
            + Has<CustomHearId<CHANNELS, BANDS>>
            + Has<Gender>
            + Has<AgeRange>
            + Clone
            + Send
            + Sync,
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
        T: Has<EqualizerConfiguration<CHANNELS, BANDS>> + Clone + Send + Sync,
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
