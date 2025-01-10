## About

OpenSCQ30 is free software for controlling settings for Soundcore headphones and earbuds. It was originally intended for the Q30's, after which the project was named, but a range of devices are now supported.

### Supported Platforms

[x] Windows - Ready  
[x] Linux - Ready  
[x] Android - Ready  
[x] Web - Ready  
[ ] macOS - It compiles. It's untested beyond that. You should probably use the web client instead. I don't have a Mac, so there's not much I can do for the desktop client. If you're tring to get it working on macOS, see [btleplug macOS build notes](https://github.com/deviceplug/btleplug#user-content-macos).

### Supported Devices

It is intended that all devices in this list work, but since I do not own them all, I can not check. Please open an issue if you can confirm that any of the devices with "Unknown" status are working correctly.

| Model    | Name              | Status  |
| -------- | ----------------- | ------- |
| A3027    | Life Q35          | Working |
| A3028    | Life Q30          | Working |
| A3029    | Life Tune         | Unknown |
| A3030    | Life Tune Pro     | Unknown |
| A3031    | Vortex            | Unknown |
| A3033    | Life 2 Neo        | Unknown |
| A3033EU  | Life 2 Neo        | Unknown |
| A3926    | Life Dot 2S       | Unknown |
| A3926Z11 | Life Dot 2S       | Unknown |
| A3930    | Liberty 2 Pro     | Unknown |
| A3931    | Life Dot 2 NC     | Unknown |
| A3931XR  | Life Dot 2 XR     | Unknown |
| A3935    | Life A2 NC        | Working |
| A3935W   | Life A2 NC        | Working |
| A3936    | Space A40         | Unknown |
| A3951    | Liberty Air 2 Pro | Unknown |
| A3945    | Life Note 3S      | Unknown |
| A3933    | Life Note 3       | Unknown |
| A3939    | Life P3           | Working |

## Installing

[![Flathub](https://img.shields.io/flathub/v/com.oppzippy.OpenSCQ30)](https://flathub.org/apps/com.oppzippy.OpenSCQ30)
[![IzzyOnDroid](https://img.shields.io/endpoint?url=https://apt.izzysoft.de/fdroid/api/v1/shield/com.oppzippy.openscq30)](https://apt.izzysoft.de/fdroid/index/apk/com.oppzippy.openscq30)

See [GitHub Releases](https://github.com/Oppzippy/OpenSCQ30/releases). All files are signed with [my GPG key](https://kylescheuing.com/publickey.txt).

## Demo

### Desktop

[Desktop GUI Demo.webm](https://user-images.githubusercontent.com/2236514/229958756-aaa2a6d3-e908-4195-aad6-b0bcfda139a8.webm)

### Android

[android-demo.webm](https://github.com/Oppzippy/OpenSCQ30/assets/2236514/2d351d63-64b8-4253-abdf-3bb5384888c1)

## Building

-   Windows: [docs/build-windows.md](docs/build-windows.md)
-   MacOS: [docs/build-macos.md](docs/build-macos.md)
-   Linux: [docs/build-linux.md](docs/build-linux.md)
-   Android: [docs/build-android.md](docs/build-android.md)

## Running Tests

`just test` will run all unit and integration tests. To run tests for a specific package, use `just gui/ test` for example.
