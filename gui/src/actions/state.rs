use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gtk::glib::JoinHandle;
use openscq30_lib::{api::device::DeviceRegistry, devices::standard::state::DeviceState};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::swappable_broadcast::SwappableBroadcast;

use super::StateUpdate;

pub struct State<T>
where
    T: DeviceRegistry + 'static,
{
    pub state_update_sender: UnboundedSender<StateUpdate>,
    pub registry: T,
    pub selected_device: RefCell<Option<Rc<T::DeviceType>>>,
    pub connect_to_device_handle: RefCell<Option<JoinHandle<()>>>,
    pub set_equalizer_configuration_handle: RefCell<Option<JoinHandle<()>>>,
    pub set_custom_noise_canceling_handle: RefCell<Option<JoinHandle<()>>>,
    pub is_refresh_in_progress: Cell<bool>,
    pub state_update_receiver: SwappableBroadcast<DeviceState>,
}

impl<T> State<T>
where
    T: DeviceRegistry + 'static,
{
    pub fn new(registry: T) -> (Rc<Self>, UnboundedReceiver<StateUpdate>) {
        let (sender, receiver) = mpsc::unbounded_channel::<StateUpdate>();
        (
            Rc::new(Self {
                connect_to_device_handle: RefCell::new(None),
                set_equalizer_configuration_handle: RefCell::new(None),
                set_custom_noise_canceling_handle: RefCell::new(None),
                selected_device: RefCell::new(None),
                is_refresh_in_progress: Cell::new(false),
                state_update_receiver: SwappableBroadcast::new(),
                registry,
                state_update_sender: sender,
            }),
            receiver,
        )
    }

    pub fn selected_device(&self) -> Option<Rc<T::DeviceType>> {
        self.selected_device.borrow().clone()
    }
}
