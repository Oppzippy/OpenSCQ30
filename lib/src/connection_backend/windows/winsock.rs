//! Fallback RFCOMM connection using Winsock. Some devices (e.g. some Soundcore Motion+ units) don't have their vendor
//! RFCOMM service exposed by WinRT's GetRfcommServicesAsync, but connecting with Winsock by service class id works
//! since Windows performs its own SDP lookup in that case.

use std::{
    mem,
    panic::Location,
    sync::{Mutex, Once},
    thread,
};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch};
use tracing::{debug, debug_span, trace};
use uuid::Uuid;
use windows::Win32::{
    Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH},
    Networking::WinSock::{
        SD_BOTH, SEND_RECV_FLAGS, SOCK_STREAM, SOCKADDR, SOCKET, WSADATA, WSAGetLastError,
        WSAStartup, closesocket, connect, recv, send, shutdown, socket,
    },
};

use crate::api::connection::{self, RfcommConnection};

use super::utils::{UuidAsGuidExt, WindowsMacAddressExt};

fn ensure_winsock_started() {
    static START: Once = Once::new();
    START.call_once(|| {
        let mut data = WSADATA::default();
        let result = unsafe { WSAStartup(0x0202, &mut data) };
        debug!("WSAStartup returned {result}");
    });
}

fn last_error(action: &'static str) -> connection::Error {
    let code = unsafe { WSAGetLastError() };
    connection::Error::Other {
        source: format!("{action} failed with winsock error {code:?}").into(),
        location: Location::caller(),
    }
}

pub struct WinsockRfcommConnection {
    socket: SOCKET,
    read_channel: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    connection_status_receiver: watch::Receiver<connection::ConnectionStatus>,
}

impl WinsockRfcommConnection {
    pub fn connect(mac_address: MacAddr6, service_uuid: Uuid) -> connection::Result<Self> {
        ensure_winsock_started();
        let socket = unsafe { socket(AF_BTH.into(), SOCK_STREAM, BTHPROTO_RFCOMM as i32) }
            .map_err(|err| connection::Error::Other {
                source: Box::new(err),
                location: Location::caller(),
            })?;

        let address = SOCKADDR_BTH {
            addressFamily: AF_BTH,
            btAddr: mac_address.as_windows_u64(),
            serviceClassId: service_uuid.as_guid(),
            // 0 means Windows looks up the channel for serviceClassId using SDP
            port: 0,
        };
        let result = unsafe {
            connect(
                socket,
                (&raw const address).cast::<SOCKADDR>(),
                mem::size_of::<SOCKADDR_BTH>() as i32,
            )
        };
        if result != 0 {
            let err = last_error("connect");
            unsafe { closesocket(socket) };
            return Err(connection::Error::DeviceNotFound {
                source: Some(Box::new(err)),
                location: Location::caller(),
            });
        }
        debug!("connected with winsock to {mac_address} service {service_uuid}");

        let (connection_status_sender, connection_status_receiver) =
            watch::channel(connection::ConnectionStatus::Connected);
        let read_channel = Self::spawn_read_channel(socket, connection_status_sender);
        Ok(Self {
            socket,
            read_channel: Mutex::new(Some(read_channel)),
            connection_status_receiver,
        })
    }

    fn spawn_read_channel(
        socket: SOCKET,
        connection_status_sender: watch::Sender<connection::ConnectionStatus>,
    ) -> mpsc::Receiver<Vec<u8>> {
        let (sender, receiver) = mpsc::channel(100);
        thread::spawn(move || {
            let span = debug_span!("WinsockRfcommConnection::read_channel");
            let _span_guard = span.enter();
            let mut buffer = [0u8; 1000];
            loop {
                trace!("waiting for inbound packet");
                let length = unsafe { recv(socket, &mut buffer, SEND_RECV_FLAGS(0)) };
                if length <= 0 {
                    debug!("recv returned {length}, socket closed");
                    break;
                }
                let packet = buffer[..length as usize].to_vec();
                trace!("received packet: {packet:?}");
                if let Err(err) = sender.blocking_send(packet) {
                    debug!("receiver is closed, aborting: {err:?}");
                    break;
                }
            }
            connection_status_sender.send_replace(connection::ConnectionStatus::Disconnected);
        });
        receiver
    }
}

#[async_trait]
impl RfcommConnection for WinsockRfcommConnection {
    async fn write(&self, data: &[u8]) -> connection::Result<()> {
        let socket = self.socket;
        let data = data.to_owned();
        tokio::task::spawn_blocking(move || -> connection::Result<()> {
            let mut remaining = data.as_slice();
            while !remaining.is_empty() {
                let written = unsafe { send(socket, remaining, SEND_RECV_FLAGS(0)) };
                if written <= 0 {
                    return Err(connection::Error::WriteError {
                        source: Some(Box::new(last_error("send"))),
                        location: Location::caller(),
                    });
                }
                remaining = &remaining[written as usize..];
            }
            trace!("wrote packet: {data:?}");
            Ok(())
        })
        .await
        .unwrap()
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.read_channel
            .lock()
            .unwrap()
            .take()
            .expect("read_channel may only be called once per WinsockRfcommConnection")
    }

    fn connection_status(&self) -> watch::Receiver<connection::ConnectionStatus> {
        self.connection_status_receiver.clone()
    }
}

impl Drop for WinsockRfcommConnection {
    fn drop(&mut self) {
        unsafe {
            shutdown(self.socket, SD_BOTH);
            closesocket(self.socket);
        }
    }
}
