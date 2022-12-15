use std::collections::VecDeque;

use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex, RwLock};

use crate::soundcore_bluetooth::traits::{
    SoundcoreDeviceConnection, SoundcoreDeviceConnectionError,
};

#[derive(Debug)]
pub struct StubSoundcoreDeviceConnection {
    name_return: RwLock<Option<Result<String, SoundcoreDeviceConnectionError>>>,
    mac_address_return: RwLock<Option<Result<String, SoundcoreDeviceConnectionError>>>,
    write_return_queue: Mutex<VecDeque<Result<(), SoundcoreDeviceConnectionError>>>,
    inbound_packets_channel:
        Mutex<Option<Result<mpsc::Receiver<Vec<u8>>, SoundcoreDeviceConnectionError>>>,
}

impl StubSoundcoreDeviceConnection {
    pub fn new() -> Self {
        StubSoundcoreDeviceConnection {
            name_return: RwLock::new(None),
            mac_address_return: RwLock::new(None),
            write_return_queue: Mutex::new(VecDeque::new()),
            inbound_packets_channel: Mutex::new(None),
        }
    }

    pub async fn set_name_return(
        &self,
        name_return: Result<String, SoundcoreDeviceConnectionError>,
    ) {
        let mut lock = self.name_return.write().await;
        *lock = Some(name_return);
    }

    pub async fn set_mac_address_return(
        &self,
        mac_address_return: Result<String, SoundcoreDeviceConnectionError>,
    ) {
        let mut lock = self.mac_address_return.write().await;
        *lock = Some(mac_address_return);
    }

    pub async fn push_write_return(
        &self,
        write_return: Result<(), SoundcoreDeviceConnectionError>,
    ) {
        let mut lock = self.write_return_queue.lock().await;
        lock.push_back(write_return);
    }

    pub async fn write_return_queue_length(&self) -> usize {
        let lock = self.write_return_queue.lock().await;
        lock.len()
    }

    pub async fn set_inbound_packets_channel(
        &self,
        receiver: Result<mpsc::Receiver<Vec<u8>>, SoundcoreDeviceConnectionError>,
    ) {
        let mut lock = self.inbound_packets_channel.lock().await;
        *lock = Some(receiver);
    }
}

#[async_trait]
impl SoundcoreDeviceConnection for StubSoundcoreDeviceConnection {
    async fn name(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        self.name_return.write().await.take().unwrap()
    }

    async fn mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        self.mac_address_return.write().await.take().unwrap()
    }

    async fn write_with_response(
        &self,
        _data: &[u8],
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let mut lock = self.write_return_queue.lock().await;
        lock.pop_front().unwrap() // we want to panic if the queue is empty so tests fail
    }

    async fn write_without_response(
        &self,
        _data: &[u8],
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let mut lock = self.write_return_queue.lock().await;
        lock.pop_front().unwrap() // we want to panic if the queue is empty so tests fail
    }

    async fn inbound_packets_channel(
        &self,
    ) -> Result<mpsc::Receiver<Vec<u8>>, SoundcoreDeviceConnectionError> {
        self.inbound_packets_channel.lock().await.take().unwrap()
    }
}
