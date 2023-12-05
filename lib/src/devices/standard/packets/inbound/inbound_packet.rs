use nom::error::VerboseError;

use crate::devices::standard::packets::parsing::{take_checksum, take_packet_header};

pub fn take_inbound_packet_body(
    input: &[u8],
) -> Result<([u8; 7], &[u8]), nom::Err<VerboseError<&[u8]>>> {
    let input = take_checksum(input)?.0;
    let (input, header) = take_packet_header(input)?;
    Ok((header.packet_type, input))
}

#[cfg(test)]
mod tests {
    use crate::devices::standard::packets::inbound::take_inbound_packet_body;
    #[test]
    fn it_errors_when_nothing_matches() {
        let result = take_inbound_packet_body(&[1, 2, 3]);
        assert_eq!(true, result.is_err());
    }
}
