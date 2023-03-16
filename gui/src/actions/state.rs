use std::{cell::RefCell, rc::Rc, sync::Arc};

use gtk::glib::JoinHandle;
use openscq30_lib::{api::device::DeviceRegistry, state::DeviceState};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::swappable_broadcast::SwappableBroadcastReceiver;

use super::StateUpdate;

pub struct State<T>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    pub state_update_sender: UnboundedSender<StateUpdate>,
    pub registry: T,
    pub selected_device: RefCell<Option<Arc<T::DeviceType>>>,
    pub connect_to_device_handle: RefCell<Option<JoinHandle<()>>>,
    pub state_update_receiver: SwappableBroadcastReceiver<DeviceState>,
}

impl<T> State<T>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    pub fn new(registry: T) -> (Rc<Self>, UnboundedReceiver<StateUpdate>) {
        let (sender, receiver) = mpsc::unbounded_channel::<StateUpdate>();
        (
            Rc::new(Self {
                connect_to_device_handle: RefCell::new(None),
                selected_device: RefCell::new(None),
                state_update_receiver: SwappableBroadcastReceiver::new(),
                registry,
                state_update_sender: sender,
            }),
            receiver,
        )
    }
}
