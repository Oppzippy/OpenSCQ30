use std::rc::Rc;

use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::EqualizerConfiguration,
};

use super::State;

pub async fn set_equalizer_configuration<T>(
    state: &Rc<State<T>>,
    equalizer_configuration: EqualizerConfiguration,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;
    device
        .set_equalizer_configuration(equalizer_configuration)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;
    use openscq30_lib::packets::structures::{EqualizerConfiguration, PresetEqualizerProfile};

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_equalizer_configuration;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);
        let mut selected_device = MockDevice::new();
        selected_device
            .expect_set_equalizer_configuration()
            .once()
            .with(predicate::eq(
                EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
            ))
            .return_once(|_ambient_sound_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Arc::new(selected_device));

        set_equalizer_configuration(
            &state,
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
        )
        .await
        .unwrap();
    }
}
