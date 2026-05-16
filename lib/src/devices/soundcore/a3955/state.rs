use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3955::{self, structures::ImmersiveExperience},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, DualBattery, DualConnections,
            DualConnectionsDevice, DualFirmwareVersion, Gender, LimitHighVolume, LowBatteryPrompt,
            SerialNumber, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3955State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    case_battery: CaseBatteryLevel,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    age_range: AgeRange,
    gender: Gender,
    hear_id: CustomHearId<2, 10>,
    button_configuration: ButtonStatusCollection<8>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3955::structures::SoundModes,
    anc_personalized_to_ear_canal: a3955::structures::AncPersonalizedToEarCanal,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    touch_tone: TouchTone,
    low_battery_prompt: LowBatteryPrompt,
    immersive_experience: ImmersiveExperience,
    dual_connections: DualConnections,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3955State {
    pub fn new(
        packet: a3955::packets::inbound::A3955StateUpdatePacket,
        dual_connections_devices: Vec<Option<DualConnectionsDevice>>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery: packet.dual_battery,
            case_battery: packet.case_battery,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            age_range: packet.age_range,
            gender: packet.gender,
            hear_id: packet.hear_id,
            button_configuration: packet.button_configuration,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            sound_modes: packet.sound_modes,
            anc_personalized_to_ear_canal: packet.anc_personalized_to_ear_canal,
            auto_power_off: packet.auto_power_off,
            limit_high_volume: packet.limit_high_volume,
            touch_tone: packet.touch_tone,
            low_battery_prompt: packet.low_battery_prompt,
            immersive_experience: packet.immersive_experience,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<a3955::packets::inbound::A3955StateUpdatePacket> for A3955State {
    fn update(&mut self, partial: a3955::packets::inbound::A3955StateUpdatePacket) {
        let a3955::packets::inbound::A3955StateUpdatePacket {
            tws_status,
            dual_battery,
            dual_firmware_version,
            serial_number,
            case_battery,
            equalizer_configuration,
            age_range,
            gender,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            anc_personalized_to_ear_canal,
            touch_tone,
            limit_high_volume,
            auto_power_off,
            low_battery_prompt,
            immersive_experience,
            dual_connections_enabled,
        } = partial;
        self.tws_status = tws_status;
        self.dual_battery = dual_battery;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.case_battery = case_battery;
        self.equalizer_configuration = equalizer_configuration;
        self.age_range = age_range;
        self.gender = gender;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.anc_personalized_to_ear_canal = anc_personalized_to_ear_canal;
        self.touch_tone = touch_tone;
        self.limit_high_volume = limit_high_volume;
        self.auto_power_off = auto_power_off;
        self.low_battery_prompt = low_battery_prompt;
        self.immersive_experience = immersive_experience;
        self.dual_connections.is_enabled = dual_connections_enabled;
    }
}
