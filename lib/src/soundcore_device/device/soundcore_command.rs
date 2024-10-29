use crate::devices::standard::state::DeviceState;

use super::Packet;

pub struct CommandResponse {
    pub packets: Vec<Packet>,
    pub new_state: DeviceState,
}
