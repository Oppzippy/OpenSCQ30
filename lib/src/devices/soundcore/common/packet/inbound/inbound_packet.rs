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
pub struct TryIntoPacketError {
    message: String,
}

impl From<TryIntoPacketError> for device::Error {
    #[track_caller]
    fn from(err: TryIntoPacketError) -> Self {
        (Box::new(err) as Box<dyn std::error::Error + Send + Sync>).into()
    }
}

pub trait TryIntoPacket<'a, 'b, T: FromPacketBody> {
    fn try_into_packet(&self) -> Result<T, TryIntoPacketError>;
    fn try_into_packet_raw_error<E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &'b self,
    ) -> Result<T, nom::Err<E>>;
}

impl<'a, 'b, T, D> TryIntoPacket<'a, 'b, T> for packet::Packet<D>
where
    'b: 'a,
    T: FromPacketBody,
    D: packet::HasDirection,
{
    fn try_into_packet(&self) -> Result<T, TryIntoPacketError> {
        self.try_into_packet_raw_error::<VerboseError<_>>()
            .map_err(|err| TryIntoPacketError {
                message: format!("{err:?}"),
            })
    }

    fn try_into_packet_raw_error<E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &'b self,
    ) -> Result<T, nom::Err<E>> {
        T::take::<E>(&self.body).map(|(_, packet)| packet)
    }
}
