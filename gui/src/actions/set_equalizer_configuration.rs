use std::time::Duration;

use gtk::glib::{self, clone, timeout_future, MainContext};
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    devices::standard::structures::EqualizerConfiguration,
};
use tokio::sync::oneshot;

use super::State;

pub async fn set_equalizer_configuration<T>(
    state: &State<T>,
    equalizer_configuration: impl Into<EqualizerConfiguration>,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    set_equalizer_configuration_impl(state, equalizer_configuration.into()).await
}

#[tracing::instrument(level = "trace", skip(state))]
async fn set_equalizer_configuration_impl<T>(
    state: &State<T>,
    equalizer_configuration: EqualizerConfiguration,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + 'static,
{
    let device = state
        .selected_device()
        .ok_or_else(|| anyhow::anyhow!("no device is selected"))?;

    // Debounce
    let (result_sender, result_receiver) = oneshot::channel::<openscq30_lib::Result<()>>();
    let new_handle = MainContext::default().spawn_local(clone!(
        #[weak]
        device,
        async move {
            timeout_future(Duration::from_millis(500)).await;
            let result = device
                .set_equalizer_configuration(equalizer_configuration)
                .await;
            result_sender.send(result).expect("receiver dropped");
        }
    ));
    if let Some(old_handle) = state
        .set_equalizer_configuration_handle
        .replace(Some(new_handle))
    {
        old_handle.abort();
    }

    match result_receiver.await {
        Ok(Ok(())) => {
            tracing::trace!("set_equalizer_configuration: returning with no error");
            Ok(())
        }
        Err(_sender_dropped) => {
            tracing::trace!("set_equalizer_configuration: sender dropped, returning");
            Ok(())
        }
        Ok(Err(err)) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, time::Duration};

    use gtk::glib::{clone, timeout_future, MainContext};
    use mockall::predicate;
    use openscq30_lib::devices::standard::structures::{
        EqualizerConfiguration, PresetEqualizerProfile,
    };

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
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        set_equalizer_configuration(
            &state,
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
        )
        .await
        .unwrap();
    }

    #[gtk::test]
    async fn it_debounces() {
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
        *state.selected_device.borrow_mut() = Some(Rc::new(selected_device));

        MainContext::default().spawn_local(clone!(
            #[strong]
            state,
            async move {
                set_equalizer_configuration(
                    &state,
                    EqualizerConfiguration::new_from_preset_profile(
                        PresetEqualizerProfile::BassReducer,
                    ),
                )
                .await
                .unwrap();
            }
        ));

        timeout_future(Duration::from_millis(50)).await;
        set_equalizer_configuration(
            &state,
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
        )
        .await
        .unwrap();
    }
}
