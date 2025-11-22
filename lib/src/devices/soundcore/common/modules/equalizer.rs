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
        soundcore::common::{
            modules::equalizer::{
                custom_equalizer_profile_store::CustomEqualizerProfileStore,
                import_export_setting_handler::ImportExportSettingHandler,
            },
            packet::PacketIOController,
            state_modifier::StateModifier,
            structures::{
                AgeRange, BasicHearId, CustomEqualizerConfiguration, CustomHearId,
                EqualizerConfiguration, Gender, TwsStatus,
            },
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

pub struct EqualizerModuleSettings {
    pub custom_preset_id: u16,
    pub band_hz: &'static [u16],
}

pub const COMMON_EQUALIZER_MODULE_SETTINGS: EqualizerModuleSettings = EqualizerModuleSettings {
    custom_preset_id: 0xfefe,
    band_hz: &[100, 200, 400, 800, 1600, 3200, 6400, 12800],
};

impl<T: 'static> ModuleCollection<T> {
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
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: false },
            )),
            COMMON_EQUALIZER_MODULE_SETTINGS,
        )
        .await;
    }

    pub async fn add_equalizer_tws<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: Has<EqualizerConfiguration<CHANNELS, BANDS>> + Has<TwsStatus> + Clone + Send + Sync,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: false },
            )),
            COMMON_EQUALIZER_MODULE_SETTINGS,
        )
        .await;
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
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: true },
            )),
            COMMON_EQUALIZER_MODULE_SETTINGS,
        )
        .await;
    }

    pub async fn add_equalizer_with_drc_tws<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: Has<EqualizerConfiguration<CHANNELS, BANDS>> + Has<TwsStatus> + Clone + Send + Sync,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: true },
            )),
            COMMON_EQUALIZER_MODULE_SETTINGS,
        )
        .await;
    }

    pub async fn add_equalizer_with_basic_hear_id_tws<
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
            + Has<TwsStatus>
            + Has<BasicHearId<CHANNELS, BANDS>>
            + Has<Gender>
            + Has<AgeRange>
            + Clone
            + Send
            + Sync,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(
                EqualizerWithBasicHearIdStateModifier::<Conn, CHANNELS, BANDS>::new(packet_io),
            ),
            COMMON_EQUALIZER_MODULE_SETTINGS,
        )
        .await;
    }

    pub async fn add_equalizer_with_custom_hear_id_tws<
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
            + Has<TwsStatus>
            + Has<CustomHearId<CHANNELS, BANDS>>
            + Has<Gender>
            + Has<AgeRange>
            + Clone
            + Send
            + Sync,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerWithCustomHearIdStateModifier::new(packet_io)),
            COMMON_EQUALIZER_MODULE_SETTINGS,
        )
        .await;
    }

    pub async fn add_equalizer_with_custom_state_modifier_tws<
        const CHANNELS: usize,
        const BANDS: usize,
        const MIN_VOLUME: i16,
        const MAX_VOLUME: i16,
        const FRACTION_DIGITS: u8,
    >(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
        state_modifier: Box<dyn StateModifier<T> + Send + Sync + 'static>,
        module_settings: EqualizerModuleSettings,
    ) where
        T: Has<
                CustomEqualizerConfiguration<
                    CHANNELS,
                    BANDS,
                    MIN_VOLUME,
                    MAX_VOLUME,
                    FRACTION_DIGITS,
                >,
            > + Has<TwsStatus>
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
            EqualizerSettingHandler::<T, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>::new(
                profile_store.to_owned(),
                module_settings.custom_preset_id,
                module_settings.band_hz,
            ).with_tws(),
        );
        self.setting_manager.add_handler(
            CategoryId::EqualizerImportExport,
            ImportExportSettingHandler::new(profile_store, change_notify),
        );
        self.state_modifiers.push(state_modifier);
    }

    pub async fn add_equalizer_with_custom_state_modifier<
        const CHANNELS: usize,
        const BANDS: usize,
        const MIN_VOLUME: i16,
        const MAX_VOLUME: i16,
        const FRACTION_DIGITS: u8,
    >(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
        state_modifier: Box<dyn StateModifier<T> + Send + Sync + 'static>,
        module_settings: EqualizerModuleSettings,
    ) where
        T: Has<
                CustomEqualizerConfiguration<
                    CHANNELS,
                    BANDS,
                    MIN_VOLUME,
                    MAX_VOLUME,
                    FRACTION_DIGITS,
                >,
            > + Clone
            + Send
            + Sync,
    {
        let profile_store = Arc::new(
            CustomEqualizerProfileStore::new(database, device_model, change_notify.to_owned())
                .await,
        );
        self.setting_manager.add_handler(
            CategoryId::Equalizer,
            EqualizerSettingHandler::<T, CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>::new(
                profile_store.to_owned(),
                module_settings.custom_preset_id,
                module_settings.band_hz,
            ),
        );
        self.setting_manager.add_handler(
            CategoryId::EqualizerImportExport,
            ImportExportSettingHandler::new(profile_store, change_notify),
        );
        self.state_modifiers.push(state_modifier);
    }
}
