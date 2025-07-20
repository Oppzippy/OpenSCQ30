use std::{sync::Arc, time::Duration};

use nom_language::error::VerboseError;
use tokio::{
    select,
    sync::{mpsc, watch},
    task::JoinHandle,
};
use tracing::trace;

use crate::{
    api::{
        connection::{ConnectionStatus, RfcommConnection},
        device,
    },
    devices::soundcore::standard::{
        packets::inbound::take_inbound_packet_header, structures::Command,
    },
};

use super::{Packet, multi_queue::MultiQueue};

pub struct PacketIOController<ConnectionType>
where
    ConnectionType: RfcommConnection,
{
    connection: Arc<ConnectionType>,
    packet_queues: Arc<MultiQueue<Command, Packet>>,
    handle: JoinHandle<()>,
}

impl<ConnectionType: RfcommConnection> Drop for PacketIOController<ConnectionType> {
    fn drop(&mut self) {
        self.handle.abort();
        trace!("dropped PacketIOController");
    }
}

impl<ConnectionType: RfcommConnection> PacketIOController<ConnectionType> {
    /// In addition to the PacketIOController, also returns a channel that all packets received
    /// that weren't a result of send_with_response will be forwarded to.
    pub async fn new(
        connection: Arc<ConnectionType>,
    ) -> device::Result<(Self, mpsc::Receiver<Packet>)> {
        let packet_queues = Arc::new(MultiQueue::new());
        let incoming_receiver = connection.read_channel();
        let (handle, outgoing_receiver) =
            Self::spawn_packet_handler(packet_queues.clone(), incoming_receiver);
        Ok((
            Self {
                connection,
                packet_queues,
                handle,
            },
            outgoing_receiver,
        ))
    }

    fn spawn_packet_handler(
        packet_queues: Arc<MultiQueue<Command, Packet>>,
        mut incoming_receiver: mpsc::Receiver<Vec<u8>>,
    ) -> (JoinHandle<()>, mpsc::Receiver<Packet>) {
        let (outgoing_sender, outgoing_receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            while let Some(bytes) = incoming_receiver.recv().await {
                let (body, header) = match take_inbound_packet_header::<VerboseError<_>>(&bytes) {
                    Ok(parsed) => parsed,
                    Err(err) => {
                        tracing::warn!("failed to parse packet: {err:?}");
                        continue;
                    }
                };
                let packet = Packet {
                    command: header,
                    body: body.to_vec(),
                };
                if !packet_queues.pop(&header, packet.clone()) {
                    match outgoing_sender.send(packet).await {
                        Ok(_) => (),
                        Err(err) => tracing::debug!(
                            "received packet that wasn't an ok, but the channel is closed, so it won't be forwarded: {err:?}"
                        ),
                    }
                }
            }
        });
        (handle, outgoing_receiver)
    }

    pub fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection.connection_status()
    }

    pub async fn send_with_response(&self, packet: &Packet) -> device::Result<Packet> {
        let queue_key = packet.command().to_inbound();
        let handle = self.packet_queues.add(queue_key);

        handle.wait_for_start().await;

        // retry
        for i in 1..=3 {
            self.connection.write(&packet.bytes()).await?;
            select! {
                _ = handle.wait_for_end() => return Ok(handle.wait_for_value().await),
                _ = tokio::time::sleep(Duration::from_millis(500 * i)) => (),
            };
        }

        self.packet_queues.cancel(&queue_key, handle);

        Err(device::Error::ActionTimedOut {
            action: "resending packet until ack received",
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        api::connection::test_stub::StubRfcommConnection,
        devices::soundcore::standard::packets::outbound::{
            OutboundPacket, SetAmbientSoundModeCyclePacket, SetSoundModePacket,
        },
    };

    use super::*;

    #[tokio::test]
    async fn test_send_multiple() {
        let (connection, sender, _receiver) = StubRfcommConnection::new();
        let controller = Arc::new(
            PacketIOController::new(Arc::new(connection))
                .await
                .unwrap()
                .0,
        );

        let handle1 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send_with_response(&SetSoundModePacket::default().into())
                    .await
                    .expect("should receive ack");
            }
        });
        let handle2 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send_with_response(&SetSoundModePacket::default().into())
                    .await
                    .expect("should receive ack");
            }
        });
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert!(!handle1.is_finished());
        assert!(!handle2.is_finished());

        sender
            .send(vec![
                0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a,
            ])
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert!(handle1.is_finished());
        assert!(!handle2.is_finished());

        sender
            .send(vec![
                0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a,
            ])
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert!(handle2.is_finished());
    }

    #[tokio::test]
    async fn test_out_of_order_responses() {
        let (connection, sender, _receiver) = StubRfcommConnection::new();
        let controller = Arc::new(
            PacketIOController::new(Arc::new(connection))
                .await
                .unwrap()
                .0,
        );

        let set_cycle_packet = SetAmbientSoundModeCyclePacket::default();
        let handle1 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send_with_response(&set_cycle_packet.into())
                    .await
                    .expect("should receive ack");
            }
        });
        let handle2 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send_with_response(&SetSoundModePacket::default().into())
                    .await
                    .expect("should receive ack");
            }
        });
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert!(!handle1.is_finished());
        assert!(!handle2.is_finished());

        sender
            .send(vec![
                0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a,
            ])
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert!(!handle1.is_finished());
        assert!(handle2.is_finished());

        sender
            .send(
                Packet {
                    command: set_cycle_packet.command().to_inbound(),
                    body: Vec::new(),
                }
                .bytes(),
            )
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert!(handle1.is_finished());
    }
}
