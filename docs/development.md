## Adding support for a new Soundcore device

### Acquiring a state update packet

Soundcore devices have most of their state packed into one big packet. In OpenSCQ30 jargon, this is referred to as a state update packet. In order to get the device to send this, a request state packet must be sent to the device. We want to read the contents of those packets so we can figure out how to parse them.

There are a few ways to proceed.

#### Wireshark (recommended)

Install the official Soundcore app and capture bluetooth packets with Wireshark. Filter for RFCOMM to see only relevant data.

Connect to the device in the Soundcore app, and record the state update packet. Then, change a setting, disconnect, and reconnect. Record the new state update packet. Compare the two, and find what changed. Repeat with all settings.

#### OpenSCQ30's Soundcore Development Device with Soundcore app

This method should only be used if you can't use Wireshark for whatever reason.

Using OpenSCQ30, add your device with the device model "Soundcore Development Device". When you connect, it will display the state update packet.

Connect to the device with OpenSCQ30 and record the state update packet. Then, disconnect with OpenSCQ30 and connect with the Soundcore app, change a setting, disconnect with the Soundcore app, and reconnect with OpenSCQ30. Record the new state update packet. Compare the two, and find what changed. Repeat with all settings.

#### soundcore-device-faker (recommended for devices you don't own)

First, acquire a state update packet either by adding the device in OpenSCQ30 with the model set to Soundcore Development Device or through Wireshark. Alternatively, have someone who does own the device provide you with the state update packet.

In the `tools/soundcore-device-faker/` directory is a small python application that will pretend to be a soundcore device and respond to packets received from the Soundcore app with predefined data. This way, you can add the device configuration in `tools/soundcore-device-faker/devices/your_device.toml` and work in reverse from the previous methods. Rather than changing something from the app and seeing how it affects the state update packet, you can change the state update packet and find what changed in the app. See the soundcore-device-faker README for usage instructions (at the time of writing, that readme has not yet been created).

### Parsing the packet

#### Common fields

##### Host Device/TWS Connected

For earbuds, the first two bytes are usually host device followed by tws connected. If host device is 0, the left earbud is the host. If host device is 1, the right earbud is the host. If tws connected is 0, only the host earbud is connected. If tws connected is 1, then both earbuds are connected. Many settings are only available when TWS earbuds are connected, although OpenSCQ30 does not currently enforce this.

##### Sound Modes

The various sound mode fields are single bytes with a value in the range of 0 through the number of options minus one. Try changing the sound mode to determine which number corresponds to which sound mode.

##### Equalizer Configuration

There are a few different variations here, but the equalizer configuration will always start with the preset id. Soundcore Signature is [0, 0] and Custom is [0xfe, 0xfe]. After that comes the db values for the 8 bands, with 120 being 0.0db (so 119 is -0.1db and 121 is +0.1db). Some devices will have data for 10 bands despite only 8 being used. For earbuds, another 8 (or 10) bands will come next, with the former applying to the left earbud and the latter the right earbud. OpenSCQ30 currently only uses the left values and ignores the right.

##### Button Actions

TODO

##### Serial number

This will be 16 bytes matching `/[0-9A-F]{16}/`. It should be relatively easy to identify due to the limited range of the values (48 to 57 and 65 to 70).

##### Firmware Version

This will be 5 bytes matching `/[0-9]{2}\.[0-9]{2}/`, so it should be easy to identify. For earbuds, it will be repeated with the former occurrence being for the left earbud and the latter for the right earbud.

### openscq30-lib implementation

1. Add the device model the `DeviceModel` enum in `lib/src/devices/device_model.rs`
2. Add a module for the device in `lib/src/devices/`
3. TODO
