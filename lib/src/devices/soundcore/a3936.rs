use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3936::{packets::A3936StateUpdatePacket, state::A3936State},
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
    A3936State,
    A3936StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3936State, A3936StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3936_sound_modes();
        builder.equalizer_with_custom_hear_id().await;
        builder.a3936_button_configuration();
        builder.ambient_sound_mode_cycle();
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestStatePacket::COMMAND,
            A3936StateUpdatePacket::default().bytes(),
        )])
    },
);

#[derive(IntoStaticStr, VariantArray)]
enum AutoPowerOffDuration {
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
    #[strum(serialize = "90m")]
    NinetyMinutes,
    #[strum(serialize = "120m")]
    OneHundredTwentyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            AutoPowerOffDuration::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            AutoPowerOffDuration::SixtyMinutes => fl!("x-minutes", minutes = 60),
            AutoPowerOffDuration::NinetyMinutes => fl!("x-minutes", minutes = 90),
            AutoPowerOffDuration::OneHundredTwentyMinutes => fl!("x-minutes", minutes = 120),
        }
    }
}
