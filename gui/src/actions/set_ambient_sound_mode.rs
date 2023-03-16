use std::rc::Rc;

use gtk::glib::{clone, MainContext};
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::AmbientSoundMode,
};

use super::State;

pub fn set_ambient_sound_mode<T>(state: &Rc<State<T>>, ambient_sound_mode_id: u8)
where
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
            // Clone the Arc and release the borrow so we can hold the value across await points safely
            device.clone()
        };
        let Some(ambient_sound_mode) = AmbientSoundMode::from_id(ambient_sound_mode_id) else {
            tracing::warn!("invalid ambient sound mode: {ambient_sound_mode_id}");
            return;
        };
        if let Err(err) = device.set_ambient_sound_mode(ambient_sound_mode).await {
            tracing::error!("error setting ambient sound mode: {err}")
        }
    }));
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;
    use openscq30_lib::packets::structures::AmbientSoundMode;

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_ambient_sound_mode;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);
        let mut selected_device = MockDevice::new();
        selected_device
            .expect_set_ambient_sound_mode()
            .once()
            .with(predicate::eq(AmbientSoundMode::NoiseCanceling))
            .return_once(|_ambient_sound_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Arc::new(selected_device));

        set_ambient_sound_mode(&state, AmbientSoundMode::NoiseCanceling.id());
    }
}
