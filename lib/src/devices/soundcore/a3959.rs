use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3959::{packets::A3959StateUpdatePacket, state::A3959State},
        standard::{
            device::fetch_state_from_state_update_packet,
            macros::soundcore_device,
            packet::outbound::{OutboundPacketBytesExt, RequestStatePacket},
        },
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3959State,
    A3959StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3959State, A3959StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3959_sound_modes();
        builder.equalizer().await;
        builder.a3959_button_configuration();
        builder.ambient_sound_mode_cycle();
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.touch_tone();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestStatePacket::COMMAND,
            A3959StateUpdatePacket::default().bytes(),
        )])
    },
);

#[derive(IntoStaticStr, VariantArray)]
enum AutoPowerOffDuration {
    #[strum(serialize = "10m")]
    TenMinutes,
    #[strum(serialize = "20m")]
    TwentyMinutes,
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            AutoPowerOffDuration::TenMinutes => fl!("x-minutes", minutes = 10),
            AutoPowerOffDuration::TwentyMinutes => fl!("x-minutes", minutes = 20),
            AutoPowerOffDuration::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            AutoPowerOffDuration::SixtyMinutes => fl!("x-minutes", minutes = 60),
        }
    }
}
