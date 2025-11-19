use crate::devices::soundcore::{a3040, common::packet};

pub fn set_button_double_press_action(
    maybe_action: Option<a3040::structures::ButtonAction>,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x04, 0x81]),
        vec![0, 0, maybe_action.map_or(0xF, |action| action as u8)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_matches_known_good_packet() {
        let packet = set_button_double_press_action(Some(a3040::structures::ButtonAction::BassUp));
        assert_eq!(
            packet.bytes(),
            vec![8, 238, 0, 0, 0, 4, 129, 13, 0, 0, 0, 7, 143]
        );
    }

    #[test]
    fn it_matches_known_good_disabled_packet() {
        let packet = set_button_double_press_action(None);
        assert_eq!(
            packet.bytes(),
            vec![8, 238, 0, 0, 0, 4, 129, 13, 0, 0, 0, 15, 151]
        );
    }
}
