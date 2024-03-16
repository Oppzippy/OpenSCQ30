TODO add details

### Lib

openscq30_lib handles the device communication and state management. On desktop, lib implements the bluetooth functionality. For other platforms (web/android), an implementation must be supplied.

### CLI

Straight forward, uses openscq30_lib in an uncomplicated manner.

### GUI

TODO

### Android

TODO

```mermaid
flowchart TD
    SoundcoreDevice["SoundcoreDevice (Kotlin)"]
    NativeSoundcoreDevice["NativeSoundcoreDevice (Rust FFI)"]
    openscq30_lib
    ManualConnection["ManualConnection (Kotlin)"]
    Headphones

    SoundcoreDevice-- Commands (protobuf) -->NativeSoundcoreDevice
    NativeSoundcoreDevice<-->openscq30_lib
    NativeSoundcoreDevice-- State Updates (protobuf) -->SoundcoreDevice
    NativeSoundcoreDevice<-- Packet Bytes -->ManualConnection
    Headphones<-- BLE Packets -->ManualConnection
```

### Web

TODO
