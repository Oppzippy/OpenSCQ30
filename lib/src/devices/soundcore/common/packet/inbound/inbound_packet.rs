use nom::{
    IResult,
    error::{ContextError, ParseError},
};
use nom_language::error::VerboseError;

use crate::{api::device, devices::soundcore::common::packet};

pub trait FromPacketBody
where
    Self: Sized,
{
    type DirectionMarker: packet::HasDirection;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E>;
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct TryToPacketError {
    message: String,
}

impl From<TryToPacketError> for device::Error {
    #[track_caller]
    fn from(err: TryToPacketError) -> Self {
        (Box::new(err) as Box<dyn std::error::Error + Send + Sync>).into()
    }
}

pub trait TryToPacket<'a, 'b, T: FromPacketBody> {
    fn try_to_packet(&self) -> Result<T, TryToPacketError>;
    fn try_to_packet_raw_error<E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &'b self,
    ) -> Result<T, nom::Err<E>>;
}

impl<'a, 'b, T, D> TryToPacket<'a, 'b, T> for packet::Packet<D>
where
    'b: 'a,
    T: FromPacketBody,
    D: packet::HasDirection,
{
    fn try_to_packet(&self) -> Result<T, TryToPacketError> {
        self.try_to_packet_raw_error::<VerboseError<_>>()
            .map_err(|err| TryToPacketError {
                message: format!("{err:?}"),
            })
    }

    fn try_to_packet_raw_error<E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &'b self,
    ) -> Result<T, nom::Err<E>> {
        T::take::<E>(&self.body).map(|(_, packet)| packet)
    }
}
