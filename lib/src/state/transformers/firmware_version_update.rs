use crate::{
    packets::inbound::FirmwareVersionUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
};

impl DeviceStateTransformer for FirmwareVersionUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        DeviceState {
            left_firmware_version: Some(self.left_firmware_version),
            right_firmware_version: Some(self.right_firmware_version),
            ..state.clone()
        }
    }
}
