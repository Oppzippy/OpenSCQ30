use crate::devices::standard::{
    packets::inbound::FirmwareVersionUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
};

impl DeviceStateTransformer for FirmwareVersionUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        DeviceState {
            firmware_version: Some(self.left_firmware_version.max(self.right_firmware_version)),
            ..state.clone()
        }
    }
}
