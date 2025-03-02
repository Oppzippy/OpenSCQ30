use std::{sync::Arc, time::Duration};

use nom::error::VerboseError;
use tokio::{select, sync::mpsc, task::JoinHandle};

use crate::{
    api::connection::Connection,
    devices::standard::{packets::inbound::take_inbound_packet_header, structures::Command},
};

use super::{Packet, multi_queue::MultiQueue};

pub struct PacketIOController<ConnectionType>
where
    ConnectionType: Connection,
{
    connection: Arc<ConnectionType>,
    packet_queues: Arc<MultiQueue<Command, Packet>>,
    handle: JoinHandle<()>,
}

impl<ConnectionType: Connection> Drop for PacketIOController<ConnectionType> {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl<ConnectionType: Connection> PacketIOController<ConnectionType> {
    pub async fn new(
        connection: Arc<ConnectionType>,
    ) -> crate::Result<(Self, mpsc::Receiver<Packet>)> {
        let packet_queues = Arc::new(MultiQueue::new());
        let incoming_receiver = connection.inbound_packets_channel().await?;
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
                packet_queues.pop(&header, Some(packet.clone()));
                match outgoing_sender.send(packet).await {
                    Ok(_) => (),
                    Err(err) => tracing::debug!(
                        "received packet that wasn't an ok, but the channel is closed, so it won't be forwarded: {err:?}"
                    ),
                }
            }
        });
        (handle, outgoing_receiver)
    }

    pub async fn send(&self, packet: &Packet) -> crate::Result<Packet> {
        let handle = self.packet_queues.add(packet.command().to_inbound());

        handle.wait_for_start().await;

        // retry
        for i in 1..=3 {
            self.connection.write_with_response(&packet.bytes()).await?;
            let result = select! {
                result = handle.wait_for_end() => result,
                _ = tokio::time::sleep(Duration::from_millis(500 * i)) => None,
            };
            if let Some(response) = result {
                return Ok(response);
            }
        }

        handle.cancel();

        Err(crate::Error::TimedOut {
            action: "resending packet until ack received",
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::sync::mpsc;

    use crate::{
        devices::standard::packets::outbound::{
            OutboundPacket, SetAmbientSoundModeCyclePacket, SetSoundModePacket,
        },
        stub::connection::StubConnection,
    };

    use super::*;

    async fn create_test_connection() -> (Arc<StubConnection>, mpsc::Sender<Vec<u8>>) {
        let connection = Arc::new(StubConnection::new());

        let (sender, receiver) = mpsc::channel(100);
        connection.set_inbound_packets_channel(Ok(receiver)).await;
        (connection, sender)
    }

    #[tokio::test]
    async fn test_send_multiple() {
        let (connection, sender) = create_test_connection().await;
        for _ in 1..10 {
            connection.push_write_return(Ok(())).await;
        }
        let controller = Arc::new(PacketIOController::new(connection).await.unwrap().0);

        let handle1 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send(&SetSoundModePacket::default().into())
                    .await
                    .expect("should receive ack");
            }
        });
        let handle2 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send(&SetSoundModePacket::default().into())
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
        let (connection, sender) = create_test_connection().await;
        for _ in 1..10 {
            connection.push_write_return(Ok(())).await;
        }
        let controller = Arc::new(PacketIOController::new(connection).await.unwrap().0);

        let set_cycle_packet = SetAmbientSoundModeCyclePacket::default();
        let handle1 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send(&set_cycle_packet.into())
                    .await
                    .expect("should receive ack");
            }
        });
        let handle2 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send(&SetSoundModePacket::default().into())
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
