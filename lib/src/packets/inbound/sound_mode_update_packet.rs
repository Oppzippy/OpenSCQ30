use nom::{combinator::map, error::context};

use crate::packets::{
    parsing::{take_sound_modes, ParseResult},
    structures::{AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundModeUpdatePacket {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
    transparency_mode: TransparencyMode,
    custom_noise_canceling: CustomNoiseCanceling,
}

pub fn take_ambient_sound_mode_update_packet(input: &[u8]) -> ParseResult<SoundModeUpdatePacket> {
    // offset 9
    context(
        "SoundModeUpdatePacket",
        map(take_sound_modes, |sound_modes| SoundModeUpdatePacket {
            ambient_sound_mode: sound_modes.ambient_sound_mode,
            noise_canceling_mode: sound_modes.noise_canceling_mode,
            transparency_mode: sound_modes.transparency_mode,
            custom_noise_canceling: sound_modes.custom_noise_canceling,
        }),
    )(input)
    // offset 13
}

impl SoundModeUpdatePacket {
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.ambient_sound_mode
    }

    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.noise_canceling_mode
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        inbound::take_ambient_sound_mode_update_packet,
        parsing::take_packet_header,
        structures::{AmbientSoundMode, NoiseCancelingMode},
    };

    #[test]
    fn it_parses_valid_packet() {
        const PACKET_BYTES: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x02, 0x01, 0x00, 0x23,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let packet = take_ambient_sound_mode_update_packet(input).unwrap().1;
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Indoor, packet.noise_canceling_mode());
    }

    #[test]
    fn it_does_not_parse_invalid_ambient_sound_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                    max value of 0x02
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x03, 0x02, 0x01, 0x00, 0x23,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let result = take_ambient_sound_mode_update_packet(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn it_does_not_parse_invalid_noise_canceling_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                          max value of 0x03
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x04, 0x01, 0x00, 0x23,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let result = take_ambient_sound_mode_update_packet(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn it_does_not_parse_unknown_packet() {
        const PACKET_BYTES: &[u8] = &[0x01, 0x02, 0x03];
        let result = take_ambient_sound_mode_update_packet(PACKET_BYTES);
        assert_eq!(true, result.is_err());
    }
}
