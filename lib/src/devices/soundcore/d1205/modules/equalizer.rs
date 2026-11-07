use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    DeviceModel,
    api::device,
    devices::soundcore::{
        common::{
            modules::{
                ModuleCollection,
                equalizer::{EqualizerModuleSettings, EqualizerPreset, InvisibleBandsMode},
            },
            packet::PacketIOController,
            state_modifier::StateModifier,
            structures::VolumeAdjustments,
        },
        d1205::{
            packets::outbound::set_equalizer,
            structures::{
                CUSTOM_EQUALIZER_PRESET_ID, D1205EqualizerConfiguration, EQUALIZER_BAND_COUNT,
                EQUALIZER_BAND_HZ, EqualizerBlock, custom_filters, preset_filters,
            },
        },
    },
    i18n::fl,
    storage::OpenSCQ30Database,
};

type D1205EqualizerPreset = EqualizerPreset<EQUALIZER_BAND_COUNT, -60, 60, 1>;

/// The device takes a list of parametric filters for presets rather than per band gains. The volume adjustments here
/// are only used for display, and are the response of the preset's filters at each band's frequency, calculated with
/// the standard RBJ biquad formulas (filter type 0 as low shelf, 1 as peaking, 2 as high shelf).
fn presets() -> Vec<D1205EqualizerPreset> {
    vec![
        EqualizerPreset {
            id: 0x0000,
            name: "SoundcoreSignature",
            localized_name: || fl!("soundcore-signature"),
            volume_adjustments: VolumeAdjustments::new([19, 7, 2, 1, 0, 0, 5, 14]),
        },
        EqualizerPreset {
            id: 0x0001,
            name: "ClearVocals",
            localized_name: || fl!("clear-vocals"),
            volume_adjustments: VolumeAdjustments::new([-11, -5, 8, 17, 22, 30, 22, 26]),
        },
        EqualizerPreset {
            id: 0x0002,
            name: "PowerfulBass",
            localized_name: || fl!("powerful-bass"),
            volume_adjustments: VolumeAdjustments::new([58, 47, 1, -2, 7, 14, 11, 3]),
        },
        EqualizerPreset {
            id: 0x0003,
            name: "CalmAndSoothing",
            localized_name: || fl!("calm-and-soothing"),
            volume_adjustments: VolumeAdjustments::new([28, 48, 4, -1, -12, -48, -2, 9]),
        },
    ]
}

fn equalizer_settings()
-> EqualizerModuleSettings<EQUALIZER_BAND_COUNT, EQUALIZER_BAND_COUNT, -60, 60, 1> {
    EqualizerModuleSettings {
        custom_preset_id: Some(CUSTOM_EQUALIZER_PRESET_ID),
        band_hz: EQUALIZER_BAND_HZ,
        presets: presets(),
        preset_min_volume: -60,
        preset_max_volume: 60,
        invisible_bands_mode: InvisibleBandsMode::Fixed(Vec::new()),
    }
}

/// Presets show their display curve, everything else shows the saved custom profile.
pub fn equalizer_configuration_from_block(block: &EqualizerBlock) -> D1205EqualizerConfiguration {
    let volume_adjustments = presets()
        .into_iter()
        .find(|preset| preset.id == block.preset_id)
        .map_or_else(
            || VolumeAdjustments::new(block.custom_volume_adjustments),
            |preset| preset.volume_adjustments,
        );
    D1205EqualizerConfiguration::new_all_bands_present(block.preset_id, [volume_adjustments])
}

impl<T> ModuleCollection<T>
where
    T: Has<D1205EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    pub async fn add_d1205_equalizer(
        &mut self,
        packet_io: Arc<PacketIOController>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) {
        self.add_equalizer_with_custom_state_modifier(
            database,
            device_model,
            change_notify,
            Box::new(EqualizerStateModifier { packet_io }),
            equalizer_settings(),
        )
        .await;
    }
}

struct EqualizerStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for EqualizerStateModifier
where
    T: Has<D1205EqualizerConfiguration> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: D1205EqualizerConfiguration = *target_state.get();
        if *state_sender.borrow().get() == target {
            return Ok(());
        }

        let packet = match preset_filters(target.preset_id()) {
            Some(filters) => set_equalizer(target.preset_id(), filters),
            None => {
                let volume_adjustments = target
                    .volume_adjustments_channel_1()
                    .copied()
                    .unwrap_or_default();
                set_equalizer(
                    CUSTOM_EQUALIZER_PRESET_ID,
                    &custom_filters(volume_adjustments.adjustments()),
                )
            }
        };
        self.packet_io.send_with_response(&packet).await?;
        state_sender.send_modify(|state| *state.get_mut() = target);
        Ok(())
    }
}
