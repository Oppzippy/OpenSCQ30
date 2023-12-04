use crate::devices::standard::state::DeviceState;

pub struct CommandResponse {
    pub packets: Vec<Vec<u8>>,
    pub new_state: DeviceState,
}
