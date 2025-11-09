use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{
            self,
            inbound::{FromPacketBody, TryToPacket},
            parsing::take_bool,
        },
        packet_manager::PacketHandler,
    },
};

pub struct FlagPacketHandler<Flag> {
    get_flag: fn(&Flag) -> bool,
    set_flag: fn(&mut Flag, bool),
}

impl<Flag> FlagPacketHandler<Flag> {
    pub fn new(get_flag: fn(&Flag) -> bool, set_flag: fn(&mut Flag, bool)) -> Self {
        Self { get_flag, set_flag }
    }
}

#[async_trait]
impl<Flag, T> PacketHandler<T> for FlagPacketHandler<Flag>
where
    T: Has<Flag> + Send + Sync,
    Flag: PartialEq,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: FlagUpdate = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let flag = state.get_mut();
            let modified = packet.0 != (self.get_flag)(flag);
            (self.set_flag)(flag, packet.0);
            modified
        });
        Ok(())
    }
}

struct FlagUpdate(pub bool);

impl FromPacketBody for FlagUpdate {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("flag update", all_consuming(map(take_bool, Self))).parse_complete(input)
    }
}
