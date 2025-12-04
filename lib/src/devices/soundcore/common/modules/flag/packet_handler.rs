use std::marker::PhantomData;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};
use openscq30_lib_has::MaybeHas;
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
        structures::Flag,
    },
};

pub struct FlagPacketHandler<FlagT> {
    _flag: PhantomData<FlagT>,
}

impl<FlagT> Default for FlagPacketHandler<FlagT> {
    fn default() -> Self {
        Self { _flag: PhantomData }
    }
}

#[async_trait]
impl<FlagT, StateT> PacketHandler<StateT> for FlagPacketHandler<FlagT>
where
    StateT: MaybeHas<FlagT> + Send + Sync,
    FlagT: Flag + PartialEq + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<StateT>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: FlagUpdate = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            // TODO rather than ignoring if maybe_get returns None, insert a Some
            if let Some(flag) = state.maybe_get_mut() {
                let modified = packet.0 != flag.get_bool();
                flag.set_bool(packet.0);
                modified
            } else {
                false
            }
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
