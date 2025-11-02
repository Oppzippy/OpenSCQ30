local KNOWN_COMMANDS = {
	[0x0101] = "State update",
	[0x0102] = "TWS status",
	[0x0103] = "Battery level",
	[0x0104] = "Battery charging",
	[0x0105] = "Firmware version and serial number",
	[0x010F] = "Chinese voice prompt",
	[0x0111] = "Game mode",
	[0x017F] = "LDAC update",
	[0x0183] = "Set touch tone",
	[0x0186] = "Set auto power off",
	[0x0281] = "Set equalizer",
	[0x0283] = "Set equalizer with DRC",
	[0x0386] = "Set equalizer and hear id (old)",
	[0x0387] = "Set equalizer and hear id (new)",
	[0x0481] = "Set button configuration of all buttons",
	[0x0482] = "Reset button configuration",
	[0x0483] = "Set button enabled",
	[0x0484] = "Set button action",
	[0x0601] = "Sound modes",
	[0x0681] = "Set sound modes",
	[0x0682] = "Set ambient sound mode cycle",
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
		subtree:add_le(field_command, buffer(start_index + 5, 2))
		subtree:add_le(field_message_length, buffer(start_index + 7, 2))
		local len = buffer(start_index + 7, 2):le_uint()
		subtree:add_le(field_body, buffer(start_index + 9, len - 10))
		subtree:add_le(field_checksum, buffer(start_index + len - 1, 1))
		start_index = start_index + len
	until start_index >= buffer:len()
end

local btrfcomm = DissectorTable.get("btrfcomm.dlci")
btrfcomm:add("0-31", soundcore_protocol)
