use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3959::{self, packets::inbound::A3959StateUpdate},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, CommonEqualizerConfiguration, DualBattery,
            DualConnections, DualConnectionsDevice, DualFirmwareVersion, GamingMode,
            LowBatteryPrompt, SerialNumber, TouchTone, TwsStatus,
            button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3959State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    button_configuration: ButtonStatusCollection<8>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3959::structures::SoundModes,
    auto_power_off: AutoPowerOff,
    touch_tone: TouchTone,
    low_battery_prompt: LowBatteryPrompt,
    dual_connections: DualConnections,
    #[has(maybe)]
    gaming_mode: Option<GamingMode>,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3959State {
    pub fn new(
        packet: a3959::packets::inbound::A3959StateUpdate,
        dual_connections_devices: Vec<Option<DualConnectionsDevice>>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery: packet.dual_battery,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            button_configuration: packet.button_configuration,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            sound_modes: packet.sound_modes,
            auto_power_off: packet.auto_power_off,
            touch_tone: packet.touch_tone,
            low_battery_prompt: packet.low_battery_prompt,
            gaming_mode: packet.gaming_mode,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3959StateUpdate> for A3959State {
    fn update(&mut self, partial: A3959StateUpdate) {
        let A3959StateUpdate {
            tws_status,
            dual_battery,
            dual_firmware_version,
            serial_number,
            equalizer_configuration,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            touch_tone,
            auto_power_off,
            low_battery_prompt,
            gaming_mode,
            dual_connections_enabled,
        } = partial;
        self.tws_status = tws_status;
        self.dual_battery = dual_battery;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.touch_tone = touch_tone;
        self.auto_power_off = auto_power_off;
        self.low_battery_prompt = low_battery_prompt;
        self.gaming_mode = gaming_mode;
        self.dual_connections.is_enabled = dual_connections_enabled;
    }
}
