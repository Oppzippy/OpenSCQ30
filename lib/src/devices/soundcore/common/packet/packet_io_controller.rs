use std::{sync::Arc, time::Duration};

use nom_language::error::VerboseError;
use tokio::{
    sync::{mpsc, watch},
    task::JoinHandle,
};
use tracing::{Instrument, info_span, trace};

use crate::{
    api::{
        connection::{ConnectionStatus, RfcommConnection},
        device,
    },
    devices::soundcore::common::packet::{self, Command},
};

use super::multi_queue::MultiQueue;

pub struct PacketIOController<ConnectionType>
where
    ConnectionType: RfcommConnection,
{
    connection: Arc<ConnectionType>,
    packet_queues: Arc<MultiQueue<Command, packet::Inbound>>,
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
    ) -> device::Result<(Self, mpsc::Receiver<packet::Inbound>)> {
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
        packet_queues: Arc<MultiQueue<Command, packet::Inbound>>,
        mut incoming_receiver: mpsc::Receiver<Vec<u8>>,
    ) -> (JoinHandle<()>, mpsc::Receiver<packet::Inbound>) {
        let (outgoing_sender, outgoing_receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            let mut buffer = Vec::<u8>::new();
            while let Some(mut bytes) = incoming_receiver.recv().await {
                buffer.extend_from_slice(&bytes);
                let mut start_index = 0;
                while start_index < buffer.len() {
                    let (remainder, packet) =
                        match packet::Inbound::take::<VerboseError<_>>(&buffer[start_index..]) {
                            Ok(parsed) => parsed,
                            Err(nom::Err::Incomplete(_)) => break,
                            Err(err) => {
                                tracing::warn!("failed to parse packet: {err:?}");
                                tracing::warn!("clearing buffer: {buffer:?}");
                                buffer.clear();
                                break;
                            }
                        };

                    tracing::debug!("received packet {packet:?}");
                    let packet_length = buffer.len() - start_index - remainder.len();
                    start_index += packet_length;

                    if !packet_queues.pop(&packet.command, packet.clone()) {
                        match outgoing_sender.send(packet).await {
                            Ok(()) => (),
                            Err(err) => tracing::debug!(
                                "received packet that wasn't an ok, but the channel is closed, so it won't be forwarded: {err:?}"
                            ),
                        }
                    }
                }
                // Reuse bytes allocation for new buffer containing the remaining partial packet
                bytes.clear();
                bytes.extend_from_slice(&buffer[start_index..]);
                buffer = bytes;
            }
        }.instrument(info_span!("packet handler")));
        (handle, outgoing_receiver)
    }

    pub fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection.connection_status()
    }

    pub async fn send_with_response(
        &self,
        packet: &packet::Outbound,
    ) -> device::Result<packet::Inbound> {
        let queue_key = packet.command;
        let handle = self.packet_queues.add(queue_key);

        handle.wait_for_start().await;

        // retry
        for i in 1..=3 {
            self.connection.write(&packet.bytes()).await?;
            if tokio::time::timeout(Duration::from_millis(500 * i), handle.wait_for_end())
                .await
                .is_ok()
            {
                return Ok(handle.wait_for_value().await);
            }
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
        devices::soundcore::common::packet::{
            self,
            outbound::{SetAmbientSoundModeCycle, SetSoundModes, ToPacket},
        },
    };

    use super::*;

    #[tokio::test(start_paused = true)]
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
                    .send_with_response(&SetSoundModes::default().to_packet())
                    .await
                    .expect("should receive ack");
            }
        });
        let handle2 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send_with_response(&SetSoundModes::default().to_packet())
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

    #[tokio::test(start_paused = true)]
    async fn test_out_of_order_responses() {
        let (connection, sender, _receiver) = StubRfcommConnection::new();
        let controller = Arc::new(
            PacketIOController::new(Arc::new(connection))
                .await
                .unwrap()
                .0,
        );

        let set_cycle_packet = SetAmbientSoundModeCycle::default().to_packet();
        let set_cycle_ack = set_cycle_packet.ack();
        let handle1 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send_with_response(&set_cycle_packet)
                    .await
                    .expect("should receive ack");
            }
        });
        let handle2 = tokio::spawn({
            let controller = controller.clone();
            async move {
                controller
                    .send_with_response(&SetSoundModes::default().to_packet())
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

        sender.send(set_cycle_ack.bytes()).await.unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;
        assert!(handle1.is_finished());
    }

    #[tokio::test(start_paused = true)]
    async fn test_fragmented_packet() {
        let (connection, sender, _receiver) = StubRfcommConnection::new();
        let packet_io = Arc::new(
            PacketIOController::new(Arc::new(connection))
                .await
                .unwrap()
                .0,
        );

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            for byte in [0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a] {
                sender.send(vec![byte]).await.unwrap();
            }
        });
        packet_io
            .send_with_response(&packet::outbound::SetSoundModes::default().to_packet())
            .await
            .expect("we should receive the fragmented ACK packet");
    }

    #[tokio::test(start_paused = true)]
    async fn test_merged_packets() {
        let (connection, sender, _receiver) = StubRfcommConnection::new();
        let packet_io = Arc::new(
            PacketIOController::new(Arc::new(connection))
                .await
                .unwrap()
                .0,
        );

        let set_sound_modes: packet::Outbound =
            packet::outbound::SetSoundModes::default().to_packet();
        let set_sound_modes_ack = set_sound_modes.ack();
        let set_ambient_sound_mode_cycle: packet::Outbound =
            packet::outbound::SetAmbientSoundModeCycle::default().to_packet();
        let set_ambient_sound_mode_cycle_ack = set_ambient_sound_mode_cycle.ack();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            let mut data = set_sound_modes_ack.bytes();
            data.extend_from_slice(&set_ambient_sound_mode_cycle_ack.bytes());
            data.extend_from_slice(&set_sound_modes_ack.bytes());
            sender.send(data).await.unwrap();
        });

        let (
            set_sound_modes_result_1,
            set_ambient_sound_mode_cycle_result,
            set_sound_modes_result_2,
        ) = tokio::join!(
            packet_io.send_with_response(&set_sound_modes),
            packet_io.send_with_response(&set_ambient_sound_mode_cycle),
            packet_io.send_with_response(&set_sound_modes),
        );
        set_sound_modes_result_1.expect("first set sound modes ack should be received");
        set_ambient_sound_mode_cycle_result
            .expect("set ambient sound mode cycle ack should be received");
        set_sound_modes_result_2.expect("second set sound modes ack should be received");
    }

    #[tokio::test(start_paused = true)]
    async fn test_garbage_data_recovery() {
        let (connection, sender, _receiver) = StubRfcommConnection::new();
        let packet_io = Arc::new(
            PacketIOController::new(Arc::new(connection))
                .await
                .unwrap()
                .0,
        );

        let set_sound_modes: packet::Outbound =
            packet::outbound::SetSoundModes::default().to_packet();
        let set_sound_modes_ack = set_sound_modes.ack();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            sender.send(vec![0; 100]).await.unwrap(); // garbage data
            // not enough time has passed to recover
            sender.send(set_sound_modes_ack.bytes()).await.unwrap();
        });

        packet_io
            .send_with_response(&set_sound_modes)
            .await
            .expect("we should recover from garbage data being sent and receive the ack");
    }
}
