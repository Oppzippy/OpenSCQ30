local KNOWN_COMMANDS = {
    [0x0101] = "State update",
    [0x0106] = "Sound modes",
    [0x017f] = "LDAC update",
    [0x01ff] = "Set LDAC",
    [0x0201] = "TWS status",
    [0x0301] = "Battery level",
    [0x0401] = "Battery charging",
    [0x0501] = "Firmware version and serial number",
    [0x0b01] = "Dual connections devices",
    [0x0b81] = "Disconnect from device via dual connections",
    [0x0b82] = "Connect to device via dual connections",
    [0x0b84] = "Set dual connections enabled",
    [0x0f01] = "Chinese voice prompt",
    [0x1101] = "Game mode",
    [0x8102] = "Set equalizer",
    [0x8104] = "Set button configuration of all buttons",
    [0x8106] = "Set sound modes",
    [0x8204] = "Reset button configuration",
    [0x8206] = "Set ambient sound mode cycle",
    [0x8301] = "Set touch tone",
    [0x8302] = "Set equalizer with DRC",
    [0x8304] = "Set button enabled",
    [0x8404] = "Set button action",
    [0x8601] = "Set auto power off",
    [0x8603] = "Set equalizer and hear id (old)",
    [0x8703] = "Set equalizer and hear id (new)",
}

local soundcore_protocol = Proto("Soundcore", "Soundcore Protocol")

local field_direction = ProtoField.bytes("soundcore.direction", "Direction", base.SPACE)
local field_command = ProtoField.uint16("soundcore.command", "Command", base.HEX, KNOWN_COMMANDS)
local field_message_length = ProtoField.uint16("soundcore.message_length", "Length")
local field_body = ProtoField.bytes("soundcore.body", "Body", base.SPACE)
local field_checksum = ProtoField.uint8("soundcore.checksum", "Checksum")
soundcore_protocol.fields = {
    field_direction,
    field_command,
    field_message_length,
    field_body,
    field_checksum,
}

-- TODO handle messages split across multiple rfcomm packets
-- by setting pinfo.desegment_len/pinfo.desegment_offset
function soundcore_protocol.dissector(buffer, pinfo, tree)
    if buffer:len() < 10 then return 0 end
    if buffer:bytes(0, 5) ~= ByteArray.new("08 ee 00 00 00") and buffer:bytes(0, 5) ~= ByteArray.new("09 ff 00 00 01") then return 0 end

    pinfo.cols.protocol = soundcore_protocol.name
    local subtree = tree:add(soundcore_protocol, buffer(), "Soundcore Protocol Data")

    local start_index = 0
    repeat
        subtree:add_le(field_direction, buffer(start_index + 0, 5))
            :append_text(buffer(0, 1):le_uint() == 8 and " (Outbound)" or " (Inbound)")
        -- big endian since that is the same order as the [u8; 2] we use in rust
        subtree:add(field_command, buffer(start_index + 5, 2))
        subtree:add_le(field_message_length, buffer(start_index + 7, 2))
        local len = buffer(start_index + 7, 2):le_uint()
        subtree:add_le(field_body, buffer(start_index + 9, len - 10))
        subtree:add_le(field_checksum, buffer(start_index + len - 1, 1))
        start_index = start_index + len
    until start_index >= buffer:len()
end

local btrfcomm = DissectorTable.get("btrfcomm.dlci")
btrfcomm:add("0-31", soundcore_protocol)
