use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    devices::standard::structures::AmbientSoundModeCycle,
};

use super::State;

pub async fn set_ambient_sound_mode_cycle<T>(
    state: &State<T>,
    cycle: AmbientSoundModeCycle,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    device.set_ambient_sound_mode_cycle(cycle).await?;
    Ok(())
}
