use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{OutboundPacketBytesExt, RequestState},
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    state::A3959State,
    packets::inbound::A3959State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, state::A3959State, packets::inbound::A3959State>(
            packet_io,
        )
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
            RequestState::COMMAND,
            packets::inbound::A3959State::default().bytes(),
        )])
    },
);

#[derive(IntoStaticStr, VariantArray)]
#[allow(clippy::enum_variant_names)]
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
            Self::TenMinutes => fl!("x-minutes", minutes = 10),
            Self::TwentyMinutes => fl!("x-minutes", minutes = 20),
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
        }
    }
}
