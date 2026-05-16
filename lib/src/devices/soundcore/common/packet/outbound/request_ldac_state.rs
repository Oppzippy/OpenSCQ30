use crate::devices::soundcore::common::packet;

pub fn request_ldac_state() -> packet::Outbound {
    packet::Outbound::new(super::REQUEST_LDAC_STATE_COMMAND, Vec::new())
}
