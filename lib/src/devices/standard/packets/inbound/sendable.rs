use nom::error::{ContextError, ParseError, VerboseError};

use crate::devices::standard::{
    packets::{parsing::take_checksum, Packet},
    structures::PacketHeader,
};

pub trait Sendable
where
    Self: Sized,
{
    fn header() -> Command;
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E>;
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct TryIntoSendableError {
    message: String,
}

impl From<TryIntoSendableError> for crate::Error {
    fn from(error: TryIntoSendableError) -> Self {
        Self::Other {
            source: Box::new(error),
        }
    }
}

pub trait TryIntoInboundPacket<'a, 'b, T: Sendable> {
    fn try_into_inbound_packet_raw_error<E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &'b self,
    ) -> Result<T, nom::Err<E>>;

    fn try_into_inbound_packet(&self) -> Result<T, TryIntoSendableError>;
}

impl<'a, 'b, T: Sendable> TryIntoInboundPacket<'a, 'b, T> for Packet
where
    'b: 'a,
{
    fn try_into_inbound_packet(&self) -> Result<T, TryIntoSendableError> {
        self.try_into_inbound_packet_raw_error::<VerboseError<_>>()
            .map_err(|err| TryIntoSendableError {
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
) -> IResult<&'a [u8], Command, E> {
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
