use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    devices::standard::structures::HearId,
};

use super::State;

#[tracing::instrument(level = "trace", skip(state))]
pub async fn set_hear_id<T>(state: &State<T>, hear_id: HearId) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    device.set_hear_id(hear_id).await?;
    Ok(())
}
