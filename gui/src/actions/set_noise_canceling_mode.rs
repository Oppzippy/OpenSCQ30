use std::rc::Rc;

use gtk::glib::{clone, MainContext};
use openscq30_lib::{
    api::device::{Device, DeviceRegistry},
    packets::structures::NoiseCancelingMode,
};

use super::State;

pub fn set_noise_canceling_mode<T>(state: &Rc<State<T>>, noise_canceling_mode_id: u8)
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
        let Some(noise_canceling_mode) = NoiseCancelingMode::from_id(noise_canceling_mode_id) else {
            tracing::error!("invalid noise canceling mode: {noise_canceling_mode_id}");
            return;
        };
        if let Err(err) = device.set_noise_canceling_mode(noise_canceling_mode).await {
            tracing::error!("error setting noise canceling mode: {err}")
        }
    }));
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;
    use openscq30_lib::packets::structures::NoiseCancelingMode;

    use crate::{
        actions::State,
        mock::{MockDevice, MockDeviceRegistry},
    };

    use super::set_noise_canceling_mode;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, _receiver) = State::new(registry);
        let mut selected_device = MockDevice::new();
        selected_device
            .expect_set_noise_canceling_mode()
            .once()
            .with(predicate::eq(NoiseCancelingMode::Transport))
            .return_once(|_noise_canceling_mode| Ok(()));
        *state.selected_device.borrow_mut() = Some(Arc::new(selected_device));

        set_noise_canceling_mode(&state, NoiseCancelingMode::Transport.id());
    }
}
