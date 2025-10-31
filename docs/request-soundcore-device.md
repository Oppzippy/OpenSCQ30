# Requesting support for a new Soundcore device

OpenSCQ30 includes a feature to gather the information I need to add support for a new device. See the instructions for your platform, and include that information when you open an issue.

## GUI

Click Add Device, then select Soundcore Development Information, and then select the device that you're requesting support for. Connect to it, and then use the copy button for State Update Packet.

## Android

Click the add device button, then select Soundcore Development Information, and then select the device that you're requesting support for. Accept the permission request, and connect to it. In the Device Information category, use the copy button next to State Update Packet.

## CLI

```bash
openscq30 paired-devices add --mac-address YOUR_SOUNDCORE_DEVICE_MAC_ADDRESS --model SoundcoreDevelopment
openscq30 device --mac-address YOUR_SOUNDCORE_DEVICE_MAC_ADDRESS setting --get stateUpdatePacket --json
```
