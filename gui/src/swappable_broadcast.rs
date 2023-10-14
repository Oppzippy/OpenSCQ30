use std::rc::Rc;

use gtk::glib::{JoinHandle, MainContext};
use tokio::sync::{watch, Mutex};

#[derive(Debug)]
pub struct SwappableBroadcast<T>
where
    T: Clone,
{
    persistent_sender_lock: Rc<Mutex<watch::Sender<Option<T>>>>,
    swapped_in_receiver_handle: Mutex<Option<JoinHandle<()>>>,
}

impl<T> SwappableBroadcast<T>
where
    T: Clone + 'static,
{
    pub fn new() -> Self {
        let (persistent_sender, _) = watch::channel::<Option<T>>(None);
        Self {
            persistent_sender_lock: Rc::new(Mutex::new(persistent_sender)),
            swapped_in_receiver_handle: Mutex::new(None),
        }
    }

    pub async fn subscribe(&self) -> watch::Receiver<Option<T>> {
        self.persistent_sender_lock.lock().await.subscribe()
    }

    pub async fn replace_receiver(&self, mut receiver: Option<watch::Receiver<T>>) {
        let mut swapped_in_receiver_handle = self.swapped_in_receiver_handle.lock().await;
        if let Some(handle) = swapped_in_receiver_handle.take() {
            handle.abort();
        }

        {
            // When we subscribe, the previous value is marked as read, so we must set it manually (receiver.changed().await will not fall through)
            let persistent_sender = self.persistent_sender_lock.lock().await;
            let maybe_value = receiver
                .as_mut()
                .map(|receiver| receiver.borrow_and_update().to_owned());
            persistent_sender.send_replace(maybe_value);
        }

        if let Some(mut receiver) = receiver {
            let persistent_sender_lock = self.persistent_sender_lock.to_owned();

            let handle = MainContext::default().spawn_local(async move {
                while let Ok(()) = receiver.changed().await {
                    let persistent_sender = persistent_sender_lock.lock().await;
                    let new_value = receiver.borrow_and_update().to_owned();
                    persistent_sender.send_replace(Some(new_value));
                }
                // The sender for the swapped in receiver closed, so swap out its value
                persistent_sender_lock.lock().await.send_replace(None);
            });
            *swapped_in_receiver_handle = Some(handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use gtk::glib::timeout_future;
    use tokio::sync::watch;

    use super::SwappableBroadcast;

    #[gtk::test]
    async fn test_swap_receiver() {
        let swappable: Arc<SwappableBroadcast<i8>> = Arc::new(SwappableBroadcast::new());
        let swappable_receiver = swappable.subscribe().await;

        {
            assert_eq!(None, *swappable_receiver.borrow());

            let (sender, receiver) = watch::channel(1);
            swappable.replace_receiver(Some(receiver)).await;
            timeout_future(Duration::from_millis(10)).await;
            assert_eq!(Some(1), *swappable_receiver.borrow());
            sender.send(2).unwrap();
            timeout_future(Duration::from_millis(10)).await;
            assert_eq!(Some(2), *swappable_receiver.borrow());
        }

        {
            let (_sender, receiver) = watch::channel(3);
            swappable.replace_receiver(Some(receiver)).await;
            timeout_future(Duration::from_millis(10)).await;
            assert_eq!(Some(3), *swappable_receiver.borrow());
        }

        swappable.replace_receiver(None).await;
        timeout_future(Duration::from_millis(10)).await;
        assert_eq!(None, *swappable_receiver.borrow());
    }

    #[gtk::test]
    async fn test_unsubscribes_from_old_receiviers() {
        let swappable: Arc<SwappableBroadcast<i8>> = Arc::new(SwappableBroadcast::new());
        let (sender, receiver) = watch::channel(1);

        swappable.replace_receiver(Some(receiver)).await;
        timeout_future(Duration::from_millis(10)).await;
        assert_eq!(1, sender.receiver_count());

        swappable.replace_receiver(None).await;
        timeout_future(Duration::from_millis(10)).await;
        assert_eq!(0, sender.receiver_count());
    }
}
