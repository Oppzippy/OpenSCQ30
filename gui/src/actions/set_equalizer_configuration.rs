use std::rc::Rc;

use gtk::glib::{clone, MainContext};
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::EqualizerConfiguration,
};

use super::State;

pub fn set_equalizer_configuration<T>(
    state: &Rc<State<T>>,
    equalizer_configuration: EqualizerConfiguration,
) where
    T: DeviceRegistry + Send + Sync + 'static,
{
    let main_context = MainContext::default();
    main_context.spawn_local(clone!(@strong state => async move {
        let device = {
            let borrow = state.selected_device.borrow();
            let Some(device) = &*borrow else {
                tracing::warn!("no device is selected");
                return;
            };
            // Clone the arc and release the borrow so we can hold the value across await points safely
            device.clone()
        };
        if let Err(err) = device.set_equalizer_configuration(equalizer_configuration).await {
            tracing::error!("error setting equalizer configuration: {err}");
        }
    }));
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
        );
    }
}
