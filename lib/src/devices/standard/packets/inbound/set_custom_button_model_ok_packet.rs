use nom::error::{ContextError, ParseError};

use crate::devices::standard::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetCustomButtonModelOkPacket {}

pub fn take_set_custom_button_model_ok_packet<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> ParseResult<SetCustomButtonModelOkPacket, E> {
    Ok((input, SetCustomButtonModelOkPacket::default()))
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::take_inbound_packet_body;

    use super::take_set_custom_button_model_ok_packet;

    #[test]
    fn it_parses_an_example_ok_packet() {
        let input: &[u8] = &[0x09, 0xFF, 0x00, 0x00, 0x01, 0x04, 0x84, 0x0A, 0x00, 0x9B];
        let (_, body) = take_inbound_packet_body(input).unwrap();
        take_set_custom_button_model_ok_packet::<VerboseError<_>>(body).expect("should not error");
    }
}
