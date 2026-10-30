use crate::devices::soundcore::common::{packet, structures::CommonEqualizerConfiguration};

/// The soundcore app sets the equalizer with the command that other devices use for DRC, but the
/// second set of bands is the right channel rather than DRC. The device does not acknowledge the
/// usual set equalizer command.
pub fn set_equalizer(
    equalizer_configuration: &CommonEqualizerConfiguration<2, 10>,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x02, 0x83]),
        equalizer_configuration.bytes().collect(),
    )
}
