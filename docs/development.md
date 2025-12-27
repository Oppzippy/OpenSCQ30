## Adding support for a new Soundcore device

### Acquiring a state update packet

Soundcore devices have most of their state packed into one big packet. In OpenSCQ30 jargon, this is referred to as a state update packet. In order to get the device to send this, a request state packet must be sent to the device. We want to read the contents of those packets so we can figure out how to parse them.

There are a few ways to proceed.

#### Wireshark (recommended)

Install the soundcore wireshark plugin from [tools/wireshark/plugins/soundcore.lua](../tools/wireshark/plugins/soundcore.lua). To do so, open Help > About > Folders in Wireshark, and open the Personal Lua Plugins directory. Copy or make a symlink to `soundcore.lua`.

Install the official Soundcore app and capture bluetooth packets with Wireshark. Filter for RFCOMM to see only relevant data.

Connect to the device in the Soundcore app, and record the state update packet. Then, change a setting, disconnect, and reconnect. Record the new state update packet. Compare the two, and find what changed. Repeat with all settings.

#### soundcore-device-faker (recommended)

First, acquire a state update packet either by adding the device in OpenSCQ30 with the model set to Soundcore Development Device or through Wireshark. Alternatively, have someone who does own the device provide you with the state update packet.

In the `tools/soundcore-device-faker/` directory is a small python application that will pretend to be a soundcore device and respond to packets received from the Soundcore app with predefined data. This way, you can add the device configuration in `tools/soundcore-device-faker/devices/your_device.toml` and work in reverse from the previous methods. Rather than changing something from the app and seeing how it affects the state update packet, you can change the state update packet and find what changed in the app. See the soundcore-device-faker README for usage instructions.

#### OpenSCQ30's Soundcore Development Device with Soundcore app (not recommended)

This method should only be used if you can't use Wireshark for whatever reason.

Using OpenSCQ30, add your device with the device model "Soundcore Development Device". When you connect, it will display the state update packet.

Connect to the device with OpenSCQ30 and record the state update packet. Then, disconnect with OpenSCQ30 and connect with the Soundcore app, change a setting, disconnect with the Soundcore app, and reconnect with OpenSCQ30. Record the new state update packet. Compare the two, and find what changed. Repeat with all settings.

### Parsing the packet

#### Common fields

##### Host Device/TWS Connected

For earbuds, the first two bytes are usually host device followed by tws connected. If host device is 0, the left earbud is the host. If host device is 1, the right earbud is the host. If tws connected is 0, only the host earbud is connected. If tws connected is 1, then both earbuds are connected. Many settings are only available when TWS earbuds are connected, although OpenSCQ30 does not currently enforce this.

##### Sound Modes

The various sound mode fields are single bytes with a value in the range of 0 through the number of options minus one. Try changing the sound mode to determine which number corresponds to which sound mode.

##### Equalizer Configuration

There are a few different variations here, but the equalizer configuration will always start with the preset id. Soundcore Signature is [0, 0] and Custom is [0xfe, 0xfe]. After that comes the db values for the 8 bands, with 120 being 0.0db (so 119 is -0.1db and 121 is +0.1db). Some devices will have data for 10 bands despite only 8 being used. For earbuds, another 8 (or 10) bands will come next, with the former applying to the left earbud and the latter the right earbud. OpenSCQ30 currently only uses the left values and ignores the right.

##### Button Actions

There are a few variations, but the general idea is that the left 4 bits refer to when TWS is not connected, and the right 4 bits refer to when TWS is connected. The button actions will be one byte for if it's enabled, and one byte for the action. So for example, [0x10, 0x23] would mean the button is enabled only when TWS is not connected, and the action is 2 when tws is disconnected and 3 when tws is connected.

Some devices don't have separate tws connected and tws disconnected for some buttons, or they don't have separate values for if the button is enabled.

##### Serial number

This will be 16 bytes matching `/[0-9A-F]{16}/`. It should be relatively easy to identify due to the limited range of the values (48 to 57 and 65 to 70).

##### Firmware Version

This will be 5 bytes matching `/[0-9]{2}\.[0-9]{2}/`, so it should be easy to identify. For earbuds, it will be repeated with the former occurrence being for the left earbud and the latter for the right earbud.

### openscq30-lib implementation

1. Add the device model the `DeviceModel` enum in `lib/src/devices/device_model.rs` and its name to `lib/i18n/en/openscq30-lib.ftl`
2. Add match arms for the new device in `DeviceModel`'s `device_registry` and `demo_device_registry` functions
3. Add a module for the device in `lib/src/devices/`
4. Create a struct for storing the device's state. Use any of the other devices as an example. The important part is `#[derive(Has)]`.
5. Create a struct for the device's state update packet, and implement `From<YourStateUpdatePacket>` for your state packet. The state update packet should implement Default, InboundPacket, and OutboundPacket. The OutboundPacket implementation is used for demo devices. Again, refer to other devices here for examples.
6. Use the `soundcore_device!` macro in the module for your device and call the relevant functions on the builder. Categories will be displayed in order of the first builder call that added the category, so make sure to properly order function calls.
