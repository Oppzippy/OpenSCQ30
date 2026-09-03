#!/usr/bin/env python3

from pathlib import Path

device_model = input("Device Model: ")
device_model = device_model.upper()

localized_device_model = input("Localized Device Model: ")

lib_dir = Path(__file__).parent.parent
devices_dir = lib_dir / "src" / "devices"
soundcore_dir = devices_dir / "soundcore"
new_device_dir = soundcore_dir / device_model.lower()
new_device_dir.mkdir()
packets_dir = new_device_dir / "packets"
packets_dir.mkdir()
inbound_dir = packets_dir / "inbound"
inbound_dir.mkdir()

# Partial file updates

def prepend_text_to_file(path: Path, text: str):
    path.write_text(text+path.read_text())

# rustfmt will sort the mod statements, so we can just put it at the beginning
prepend_text_to_file(devices_dir / "soundcore.rs", f"pub mod {device_model.lower()};\n")

prepend_text_to_file(lib_dir / "i18n" / "en" / "openscq30-lib.ftl", f"soundcore-{device_model.lower()} = {localized_device_model}\n")

def add_device_model_to_enum(device_model: str):
    path = devices_dir / "device_model.rs"
    lines = path.read_text().splitlines()

    new_enum_variant= f"    Soundcore{device_model},"

    # insert the variant at a location to ensure things remain sorted
    enum_line_number = lines.index("pub enum DeviceModel {")
    enum_end_line_number = lines.index("}", enum_line_number)
    target_line_number = enum_line_number + 1
    for i in range(target_line_number, enum_end_line_number):
        if lines[i] < new_enum_variant:
            target_line_number = i + 1
        else:
            break

    lines.insert(target_line_number, new_enum_variant)
    path.write_text('\n'.join(lines))

add_device_model_to_enum(device_model)

# Full files

(new_device_dir / "packets.rs").write_text(f"""
pub mod inbound;
""".lstrip())

(packets_dir / "inbound.rs").write_text(f"""
mod state_update;

pub use state_update::*;
""".lstrip())


(new_device_dir / "state.rs").write_text(f"""
use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::structures::SerialNumber;

use super::packets::inbound::{device_model}StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct {device_model}State {{
    serial_number: SerialNumber,
}}

impl From<{device_model}StateUpdatePacket> for {device_model}State {{
    fn from(value: {device_model}StateUpdatePacket) -> Self {{
        Self {{
            serial_number: value.serial_number,
        }}
    }}
}}
""".lstrip())

(inbound_dir / "state_update.rs").write_text(f"""
use nom::{{
    IResult, Parser,
    combinator::{{map, opt}},
    error::{{ContextError, ParseError, context}},
}};

use crate::{{
    api::device,
    devices::soundcore::{{
        {device_model.lower()}::state::{device_model}State,
        common::{{
            macros::state_update_packet_module,
            packet::{{
                self, Command,
                inbound::FromPacketBody,
                outbound::ToPacket,
            }},
            structures::SerialNumber,
        }},
    }},
}};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct {device_model}StateUpdatePacket {{
    pub serial_number: SerialNumber,
}}

impl FromPacketBody for {device_model}StateUpdatePacket {{
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {{
        context(
            "{device_model.lower()} state update packet",
            map(
                (
                    SerialNumber::take,
                ),
                |(
                    serial_number,
                )| {{
                    Self {{
                        serial_number,
                    }}
                }},
            ),
        )
        .parse_complete(input)
    }}
}}

impl ToPacket for {device_model}StateUpdatePacket {{
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {{
        packet::inbound::STATE_COMMAND
    }}

    fn body(&self) -> Vec<u8> {{
        self.serial_number.bytes().into_iter().collect()
    }}
}}

state_update_packet_module!({device_model}State, {device_model}StateUpdatePacket);
""".lstrip())

(soundcore_dir / f"{device_model.lower()}.rs").write_text(f"""
use std::collections::HashMap;

use crate::devices::soundcore::{{
    {device_model.lower()}::{{packets::inbound::{device_model}StateUpdatePacket, state::{device_model}State}},
    common::{{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{{RequestState, ToPacket}},
    }},
}};

mod packets;
mod state;

soundcore_device!(
    {device_model}State,
    async |packet_io| {{
        fetch_state_from_state_update_packet::<{device_model}State, {device_model}StateUpdatePacket>(packet_io)
            .await
    }},
    async |builder| {{
        builder.module_collection().add_state_update();
    }},
    {{
        HashMap::from([(
            RequestState::COMMAND,
            {device_model}StateUpdatePacket::default().to_packet(),
        )])
    }},
);
""".lstrip())
