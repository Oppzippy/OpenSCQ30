use tokio::{
    select,
    sync::{broadcast, mpsc, Mutex},
};
use tracing::error;

#[derive(Debug)]
pub struct SwappableBroadcastReceiver<T>
where
    T: Clone + Copy,
{
    receiver: Mutex<Option<broadcast::Receiver<T>>>,
    new_receiver_receiver: Mutex<mpsc::Receiver<Option<broadcast::Receiver<T>>>>,
    new_receiver_sender: mpsc::Sender<Option<broadcast::Receiver<T>>>,
}

impl<T> SwappableBroadcastReceiver<T>
where
    T: Clone + Copy,
{
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(10);
        Self {
            receiver: Mutex::new(None),
            new_receiver_receiver: Mutex::new(receiver),
            new_receiver_sender: sender,
        }
    }

    pub async fn next(&self) -> T {
        let mut maybe_receiver = self.receiver.lock().await;
        let mut new_receiver_receiver = self.new_receiver_receiver.lock().await;
        loop {
            if let Some(receiver) = maybe_receiver.as_mut() {
                let result = select! {
                    val = receiver.recv() => {
                        Some(val)
                    }
                    maybe_new_receiver = new_receiver_receiver.recv() => {
                        if let Some(new_receiver) = maybe_new_receiver {
                            *maybe_receiver = new_receiver;
                        } else {
                            *maybe_receiver = None;
                        }
                        None
                    }
                };
                match result {
                    Some(Ok(value)) => return value,
                    Some(Err(err)) => match err {
                        broadcast::error::RecvError::Closed => {
                            *maybe_receiver = None;
                            tracing::debug!("receiver closed, removing");
                        }
                        broadcast::error::RecvError::Lagged(_) => {
                            error!("error reading from broadcast receiver: {err}")
                        }
                    },
                    None => (), // receiver is swapped out, try again
                }
            } else if let Some(new_receiver) = new_receiver_receiver.recv().await {
                *maybe_receiver = new_receiver;
            } else {
                *maybe_receiver = None;
            }
        }
    }

    pub async fn replace_receiver(&self, receiver: Option<broadcast::Receiver<T>>) {
        self.new_receiver_sender.send(receiver).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use tokio::sync::{broadcast, Mutex};

    use super::SwappableBroadcastReceiver;

    #[tokio::test]
    async fn test_swap_receiver() {
        let swappable: Arc<SwappableBroadcastReceiver<i8>> =
            Arc::new(SwappableBroadcastReceiver::new());

        let values = Arc::new(Mutex::new(Vec::new()));
        {
            let swappable = swappable.to_owned();
            let values = values.to_owned();
            tokio::spawn(async move {
                loop {
                    let value = swappable.next().await;
                    values.lock().await.push(value);
                }
            });
        }

        let (sender1, receiver1) = broadcast::channel(5);
        swappable.replace_receiver(Some(receiver1)).await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        sender1.send(0).unwrap();
        sender1.send(1).unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        let (sender2, receiver2) = broadcast::channel(5);
        swappable.replace_receiver(Some(receiver2)).await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        sender2.send(2).unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        swappable.replace_receiver(None).await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(sender2.send(3).is_err());

        assert_eq!(*values.lock().await, vec![0, 1, 2]);
    }
}
