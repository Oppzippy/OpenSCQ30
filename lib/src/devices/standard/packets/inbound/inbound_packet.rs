use nom::error::{ContextError, ParseError, VerboseError};

use crate::{
    devices::standard::{
        packets::parsing::{take_checksum, ParseResult},
        structures::PacketHeader,
    },
    soundcore_device::device::Packet,
};

pub trait InboundPacket
where
    Self: Sized,
{
    fn header() -> [u8; 7];
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<Self, E>;
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct TryIntoInboundPacketError {
    message: String,
}

impl From<TryIntoInboundPacketError> for crate::Error {
    fn from(error: TryIntoInboundPacketError) -> Self {
        Self::Other {
            source: Box::new(error),
        }
    }
}

pub trait TryIntoInboundPacket<'a, 'b, T: InboundPacket> {
    fn try_into_inbound_packet_raw_error<E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &'b self,
    ) -> Result<T, nom::Err<E>>;

    fn try_into_inbound_packet(&self) -> Result<T, TryIntoInboundPacketError>;
}

impl<'a, 'b, T: InboundPacket> TryIntoInboundPacket<'a, 'b, T> for Packet
where
    'b: 'a,
{
    fn try_into_inbound_packet(&self) -> Result<T, TryIntoInboundPacketError> {
        self.try_into_inbound_packet_raw_error::<VerboseError<_>>()
            .map_err(|err| TryIntoInboundPacketError {
                message: format!("{err:?}"),
            })
    }

    fn try_into_inbound_packet_raw_error<E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &'b self,
    ) -> Result<T, nom::Err<E>> {
        T::take::<E>(&self.body).map(|(_, packet)| packet)
    }
}

pub(crate) fn take_inbound_packet_header<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<[u8; 7], E> {
    let input = take_checksum(input)?.0;
    let (input, header) = PacketHeader::take(input)?;
    Ok((input, header.packet_type))
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::take_inbound_packet_header;
    #[test]
    fn it_errors_when_nothing_matches() {
        let result = take_inbound_packet_header::<VerboseError<_>>(&[1, 2, 3]);
        assert_eq!(true, result.is_err());
    }
}
