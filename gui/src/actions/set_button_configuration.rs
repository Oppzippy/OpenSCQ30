use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    devices::standard::structures::MultiButtonConfiguration,
};

use super::State;

#[tracing::instrument(level = "trace", skip(state))]
pub async fn set_button_configuration<T>(
    state: &State<T>,
    custom_button_model: MultiButtonConfiguration,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    device
        .set_multi_button_configuration(custom_button_model)
        .await?;
    Ok(())
}
