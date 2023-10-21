use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::CustomButtonModel,
};

use super::State;

#[tracing::instrument(level = "trace", skip(state))]
pub async fn set_custom_button_model<T>(
    state: &State<T>,
    custom_button_model: CustomButtonModel,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    device.set_custom_button_model(custom_button_model).await?;
    Ok(())
}
