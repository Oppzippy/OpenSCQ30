use std::sync::Arc;

use tokio::{
    select,
    sync::{broadcast, Mutex, Notify},
};
use tracing::error;

#[derive(Debug)]
pub struct SwappableBroadcastReceiver<T>
where
    T: Clone + Copy,
{
    receiver: Mutex<Option<broadcast::Receiver<T>>>,
    new_receiver: Arc<Mutex<Option<broadcast::Receiver<T>>>>,
    notify: Arc<Notify>,
}

impl<T> SwappableBroadcastReceiver<T>
where
    T: Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            receiver: Mutex::new(None),
            new_receiver: Arc::new(Mutex::new(None)),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn next(&self) -> T {
        let mut maybe_receiver = self.receiver.lock().await;
        loop {
            if maybe_receiver.is_some() {
                let receiver = maybe_receiver.as_mut().unwrap();
                let result = select! {
                    val = receiver.recv() => {
                        Some(val)
                    }
                    _ = self.notify.notified() => {
                        let new_receiver = self.new_receiver.lock().await;
                        *maybe_receiver = new_receiver.as_ref().map(|r| r.resubscribe());
                        None
                    }
                };
                match result {
                    Some(Ok(value)) => return value,
                    Some(Err(err)) => error!("error reading from broadcast receiver: {err}"),
                    None => (), // receiver is swapped out, try again
                }
            } else {
                // If no receiver is present, wait for one to come in
                self.notify.notified().await;
                let new_receiver = self.new_receiver.lock().await;
                *maybe_receiver = new_receiver.as_ref().map(|r| r.resubscribe());
            }
        }
    }

    pub async fn replace_receiver(&self, receiver: broadcast::Receiver<T>) {
        // Drop lock before notifying
        {
            let mut new_receiver = self.new_receiver.lock().await;
            *new_receiver = Some(receiver);
        }
        self.notify.notify_waiters();
    }
}
