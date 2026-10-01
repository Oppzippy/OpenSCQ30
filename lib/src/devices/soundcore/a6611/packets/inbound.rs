use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{map, rest},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a6611::{state::A6611State, structures::BluetoothMode},
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::{DualBatteryLevel, DualFirmwareVersion, SerialNumber, TwsStatus},
    },
};

// Sent by the earbuds whenever the mode changes, including from a double tap. Body is the same as the set command.
pub const BLUETOOTH_MODE_UPDATE_COMMAND: packet::Command = packet::Command([0x01, 0x14]);

// Equalizer, volume adjustments, buttons and next alarm, see tools/soundcore-device-faker/devices/a6611.toml
const UNDECODED_BEFORE_BLUETOOTH_MODE_LEN: usize = 35;

// Layout reference: tools/soundcore-device-faker/devices/a6611.toml
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A6611StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBatteryLevel,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub undecoded_before_bluetooth_mode: Vec<u8>,
    pub bluetooth_mode: BluetoothMode,
    // Undecoded remainder (sleep settings, prompts, case battery)
    pub unknown: Vec<u8>,
}

impl Default for A6611StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            undecoded_before_bluetooth_mode: vec![0; UNDECODED_BEFORE_BLUETOOTH_MODE_LEN],
            bluetooth_mode: Default::default(),
            unknown: Vec::new(),
        }
    }
}

impl FromPacketBody for A6611StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a6611 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBatteryLevel::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    take(UNDECODED_BEFORE_BLUETOOTH_MODE_LEN),
                    BluetoothMode::take,
                    rest,
                ),
                |(
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    undecoded_before_bluetooth_mode,
                    bluetooth_mode,
                    unknown,
                ): (
                    TwsStatus,
                    DualBatteryLevel,
                    DualFirmwareVersion,
                    SerialNumber,
                    &[u8],
                    BluetoothMode,
                    &[u8],
                )| Self {
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    undecoded_before_bluetooth_mode: undecoded_before_bluetooth_mode.to_vec(),
                    bluetooth_mode,
                    unknown: unknown.to_vec(),
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A6611StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.undecoded_before_bluetooth_mode.iter().copied())
            .chain(self.bluetooth_mode.bytes())
            .chain(self.unknown.iter().copied())
            .collect()
    }
}

state_update_packet_module!(A6611State, A6611StateUpdatePacket);
