mod api;
mod packets;
mod soundcore_bluetooth;

use std::error::Error;

use ::btleplug::platform::Manager;

use crate::{
    packets::{
        outbound::outbound_packet::OutboundPacket,
        outbound::set_ambient_mode::SetAmbientSoundModePacket,
        structures::{
            ambient_sound_mode::AmbientSoundMode, noise_canceling_mode::NoiseCancelingMode,
        },
    },
    soundcore_bluetooth::{
        btleplug::soundcore_device_connection_registry::BtlePlugSoundcoreDeviceConnectionRegistry,
        traits::soundcore_device_connection_registry::SoundcoreDeviceConnectionRegistry,
    },
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let packet = SetAmbientSoundModePacket::new(
        AmbientSoundMode::Transparency,
        NoiseCancelingMode::Transport,
    );

    let manager = Manager::new().await?;
    let mut handler = BtlePlugSoundcoreDeviceConnectionRegistry::new(manager);
    handler.refresh_connections().await?;

    let devices = handler.get_connections().await;
    devices
        .first()
        .unwrap()
        .write_with_response(&packet.bytes())
        .await?;
    println!("Done");
    Ok(())
}
