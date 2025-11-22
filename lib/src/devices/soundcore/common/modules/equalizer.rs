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
                AgeRange, BasicHearId, CommonEqualizerConfiguration, CustomHearId,
                EqualizerConfiguration, Gender, TwsStatus, VolumeAdjustments,
            },
        },
    },
    i18n::fl,
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

pub struct EqualizerModuleSettings<
    const VISIBLE_BANDS: usize,
    const PRESET_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    pub custom_preset_id: u16,
    pub band_hz: [u16; VISIBLE_BANDS],
    pub presets: Vec<EqualizerPreset<PRESET_BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>,
}

#[derive(Clone, Copy, Debug)]
pub struct EqualizerPreset<
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    pub id: u16,
    pub name: &'static str,
    pub localized_name: fn() -> String,
    pub volume_adjustments: VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>,
}

pub fn common_equalizer_module_settings() -> EqualizerModuleSettings<8, 8, -120, 134, 1> {
    EqualizerModuleSettings {
        custom_preset_id: 0xfefe,
        band_hz: [100, 200, 400, 800, 1600, 3200, 6400, 12800],
        presets: vec![
            EqualizerPreset {
                name: "SoundcoreSignature",
                localized_name: || fl!("soundcore-signature"),
                id: 0x0000,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "Acoustic",
                localized_name: || fl!("acoustic"),
                id: 0x0001,
                volume_adjustments: VolumeAdjustments::new([40, 10, 20, 20, 40, 40, 40, 20]),
            },
            EqualizerPreset {
                name: "BassBooster",
                localized_name: || fl!("bass-booster"),
                id: 0x0002,
                volume_adjustments: VolumeAdjustments::new([40, 30, 10, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "BassReducer",
                localized_name: || fl!("bass-reducer"),
                id: 0x0003,
                volume_adjustments: VolumeAdjustments::new([-40, -30, -10, 0, 0, 0, 0, 0]),
            },
            EqualizerPreset {
                name: "Classical",
                localized_name: || fl!("classical"),
                id: 0x0004,
                volume_adjustments: VolumeAdjustments::new([30, 30, -20, -20, 0, 20, 30, 40]),
            },
            EqualizerPreset {
                name: "Podcast",
                localized_name: || fl!("podcast"),
                id: 0x0005,
                volume_adjustments: VolumeAdjustments::new([-30, 20, 40, 40, 30, 20, 0, -20]),
            },
            EqualizerPreset {
                name: "Dance",
                localized_name: || fl!("dance"),
                id: 0x0006,
                volume_adjustments: VolumeAdjustments::new([20, -30, -10, 10, 20, 20, 10, -30]),
            },
            EqualizerPreset {
                name: "Deep",
                localized_name: || fl!("deep"),
                id: 0x0007,
                volume_adjustments: VolumeAdjustments::new([20, 10, 30, 30, 20, -20, -40, -50]),
            },
            EqualizerPreset {
                name: "Electronic",
                localized_name: || fl!("electronic"),
                id: 0x0008,
                volume_adjustments: VolumeAdjustments::new([30, 20, -20, 20, 10, 20, 30, 30]),
            },
            EqualizerPreset {
                name: "Flat",
                localized_name: || fl!("flat"),
                id: 0x0009,
                volume_adjustments: VolumeAdjustments::new([-20, -20, -10, 0, 0, 0, -20, -20]),
            },
            EqualizerPreset {
                name: "HipHop",
                localized_name: || fl!("hip-hop"),
                id: 0x000a,
                volume_adjustments: VolumeAdjustments::new([20, 30, -10, -10, 20, -10, 20, 30]),
            },
            EqualizerPreset {
                name: "Jazz",
                localized_name: || fl!("jazz"),
                id: 0x000b,
                volume_adjustments: VolumeAdjustments::new([20, 20, -20, -20, 0, 20, 30, 40]),
            },
            EqualizerPreset {
                name: "Latin",
                localized_name: || fl!("latin"),
                id: 0x000c,
                volume_adjustments: VolumeAdjustments::new([0, 0, -20, -20, -20, 0, 30, 50]),
            },
            EqualizerPreset {
                name: "Lounge",
                localized_name: || fl!("lounge"),
                id: 0x000d,
                volume_adjustments: VolumeAdjustments::new([-10, 20, 40, 30, 0, -20, 20, 10]),
            },
            EqualizerPreset {
                name: "Piano",
                localized_name: || fl!("piano"),
                id: 0x000e,
                volume_adjustments: VolumeAdjustments::new([0, 30, 30, 20, 40, 50, 30, 40]),
            },
            EqualizerPreset {
                name: "Pop",
                localized_name: || fl!("pop"),
                id: 0x000f,
                volume_adjustments: VolumeAdjustments::new([-10, 10, 30, 30, 10, -10, -20, -30]),
            },
            EqualizerPreset {
                name: "RnB",
                localized_name: || fl!("rnb"),
                id: 0x0010,
                volume_adjustments: VolumeAdjustments::new([60, 20, -20, -20, 20, 30, 30, 40]),
            },
            EqualizerPreset {
                name: "Rock",
                localized_name: || fl!("rock"),
                id: 0x0011,
                volume_adjustments: VolumeAdjustments::new([30, 20, -10, -10, 10, 30, 30, 30]),
            },
            EqualizerPreset {
                name: "SmallSpeakers",
                localized_name: || fl!("small-speakers"),
                id: 0x0012,
                volume_adjustments: VolumeAdjustments::new([40, 30, 10, 0, -20, -30, -40, -40]),
            },
            EqualizerPreset {
                name: "SpokenWord",
                localized_name: || fl!("spoken-word"),
                id: 0x0013,
                volume_adjustments: VolumeAdjustments::new([-30, -20, 10, 20, 20, 10, 0, -30]),
            },
            EqualizerPreset {
                name: "TrebleBooster",
                localized_name: || fl!("treble-booster"),
                id: 0x0014,
                volume_adjustments: VolumeAdjustments::new([-20, -20, -20, -10, 10, 20, 20, 40]),
            },
            EqualizerPreset {
                name: "TrebleReducer",
                localized_name: || fl!("treble-reducer"),
                id: 0x0015,
                volume_adjustments: VolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60]),
            },
        ],
    }
}

impl<T: 'static> ModuleCollection<T> {
    pub async fn add_equalizer<Conn, const CHANNELS: usize, const BANDS: usize>(
        &mut self,
        packet_io: Arc<PacketIOController<Conn>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) where
        Conn: RfcommConnection + 'static + Send + Sync,
        T: Has<CommonEqualizerConfiguration<CHANNELS, BANDS>> + Clone + Send + Sync,
    {
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: false },
            )),
            common_equalizer_module_settings(),
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
        T: Has<CommonEqualizerConfiguration<CHANNELS, BANDS>>
            + Has<TwsStatus>
            + Clone
            + Send
            + Sync,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: false },
            )),
            common_equalizer_module_settings(),
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
        T: Has<CommonEqualizerConfiguration<CHANNELS, BANDS>> + Clone + Send + Sync,
    {
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: true },
            )),
            common_equalizer_module_settings(),
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
        T: Has<CommonEqualizerConfiguration<CHANNELS, BANDS>>
            + Has<TwsStatus>
            + Clone
            + Send
            + Sync,
    {
        self.add_equalizer_with_custom_state_modifier_tws(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier::new(
                packet_io,
                EqualizerStateModifierOptions { has_drc: true },
            )),
            common_equalizer_module_settings(),
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
        T: Has<CommonEqualizerConfiguration<CHANNELS, BANDS>>
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
            common_equalizer_module_settings(),
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
        T: Has<CommonEqualizerConfiguration<CHANNELS, BANDS>>
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
            common_equalizer_module_settings(),
        )
        .await;
    }

    pub async fn add_equalizer_with_custom_state_modifier_tws<
        const CHANNELS: usize,
        const BANDS: usize,
        const VISIBLE_BANDS: usize,
        const PRESET_BANDS: usize,
        const MIN_VOLUME: i16,
        const MAX_VOLUME: i16,
        const FRACTION_DIGITS: u8,
    >(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
        state_modifier: Box<dyn StateModifier<T> + Send + Sync + 'static>,
        module_settings: EqualizerModuleSettings<
            VISIBLE_BANDS,
            PRESET_BANDS,
            MIN_VOLUME,
            MAX_VOLUME,
            FRACTION_DIGITS,
        >,
    ) where
        T: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
            + Has<TwsStatus>
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
            EqualizerSettingHandler::<
                T,
                CHANNELS,
                BANDS,
                VISIBLE_BANDS,
                PRESET_BANDS,
                MIN_VOLUME,
                MAX_VOLUME,
                FRACTION_DIGITS,
            >::new(
                profile_store.to_owned(),
                module_settings.custom_preset_id,
                module_settings.band_hz,
                module_settings.presets,
            )
            .with_tws(),
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
        const VISIBLE_BANDS: usize,
        const PRESET_BANDS: usize,
        const MIN_VOLUME: i16,
        const MAX_VOLUME: i16,
        const FRACTION_DIGITS: u8,
    >(
        &mut self,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
        state_modifier: Box<dyn StateModifier<T> + Send + Sync + 'static>,
        module_settings: EqualizerModuleSettings<
            VISIBLE_BANDS,
            PRESET_BANDS,
            MIN_VOLUME,
            MAX_VOLUME,
            FRACTION_DIGITS,
        >,
    ) where
        T: Has<EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>>
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
            EqualizerSettingHandler::<
                T,
                CHANNELS,
                BANDS,
                VISIBLE_BANDS,
                PRESET_BANDS,
                MIN_VOLUME,
                MAX_VOLUME,
                FRACTION_DIGITS,
            >::new(
                profile_store.to_owned(),
                module_settings.custom_preset_id,
                module_settings.band_hz,
                module_settings.presets,
            ),
        );
        self.setting_manager.add_handler(
            CategoryId::EqualizerImportExport,
            ImportExportSettingHandler::new(profile_store, change_notify),
        );
        self.state_modifiers.push(state_modifier);
    }
}
