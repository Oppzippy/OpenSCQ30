## This branch is for v2, which is a work in progress. See the v1 branch.

[v1 branch here](https://github.com/Oppzippy/OpenSCQ30/tree/v1)

## About

OpenSCQ30 is free software for controlling settings for Soundcore headphones and earbuds. It was originally intended for the Q30's, after which the project was named, but a range of devices are now supported.

### Supported Platforms

[x] Windows - Ready  
[x] Linux - Ready  
[x] Android - Ready

### Supported Devices

| Model | Name                               |
| ----- | ---------------------------------- |
| A3004 | Soundcore Q20i                     |
| A3027 | Soundcore Life Q35                 |
| A3028 | Soundcore Life Q30                 |
| A3029 | Soundcore Life Tune                |
| A3030 | Soundcore Life Tune Pro            |
| A3031 | Soundcore Vortex                   |
| A3033 | Soundcore Life 2 Neo               |
| A3040 | Soundcore Space Q45                |
| A3116 | Soundcore Motion+                  |
| A3926 | Soundcore Life Dot 2S              |
| A3930 | Soundcore Liberty 2 Pro            |
| A3931 | Soundcore Life Dot 2 NC            |
| A3933 | Soundcore Life Note 3              |
| A3935 | Soundcore Life A2 NC               |
| A3936 | Soundcore Space A40                |
| A3939 | Soundcore Life P3                  |
| A3945 | Soundcore Life Note 3S             |
| A3947 | Soundcore Liberty 4 NC             |
| A3948 | Soundcore A20i                     |
| A3951 | Soundcore Liberty Air 2 Pro        |
| A3959 | Soundcore P30i / Soundcore R50i NC |

### Requested Devices

| Model | Name                    | Issue                                                      |
| ----- | ----------------------- | ---------------------------------------------------------- |
| A3035 | Soundcore Space One     | [GH-140](https://github.com/Oppzippy/OpenSCQ30/issues/140) |
| A3910 | Soundcore Liberty Air 2 | [GH-128](https://github.com/Oppzippy/OpenSCQ30/issues/128) |
| A3949 | Soundcore P20i          | [GH-108](https://github.com/Oppzippy/OpenSCQ30/issues/108) |
| A3952 | Soundcore Liberty 3 Pro | [GH-134](https://github.com/Oppzippy/OpenSCQ30/issues/134) |
| A3954 | Soundcore Liberty 4 Pro | [GH-169](https://github.com/Oppzippy/OpenSCQ30/issues/169) |
| A3955 | Soundcore P40i          | [GH-159](https://github.com/Oppzippy/OpenSCQ30/issues/159) |
| A3982 | Soundcore Dot 3i        | [GH-147](https://github.com/Oppzippy/OpenSCQ30/issues/147) |
| A6611 | Soundcore Sleep A20     | [GH-165](https://github.com/Oppzippy/OpenSCQ30/issues/165) |

## Installing

See [GitHub Releases](https://github.com/Oppzippy/OpenSCQ30/releases). All files are signed with [my GPG key](https://kylescheuing.com/publickey.txt).

[![Flathub](https://img.shields.io/flathub/v/com.oppzippy.OpenSCQ30)](https://flathub.org/apps/com.oppzippy.OpenSCQ30)
[![IzzyOnDroid](https://img.shields.io/endpoint?url=https://apt.izzysoft.de/fdroid/api/v1/shield/com.oppzippy.openscq30)](https://apt.izzysoft.de/fdroid/index/apk/com.oppzippy.openscq30)

[![Packaging status](https://repology.org/badge/vertical-allrepos/openscq30.svg)](https://repology.org/project/openscq30/versions)

## Contributing

### Code

See [docs/development.md](docs/development.md)

### Translations

[![Translation status](https://translate.codeberg.org/widget/openscq30/multi-auto.svg)](https://translate.codeberg.org/engage/openscq30/)

## Demo

### Desktop

[desktop-demo.webm](https://github.com/user-attachments/assets/3df615f5-2e5d-44e8-9604-f5175c11ea5b)

### Android

[android-demo.webm](https://github.com/user-attachments/assets/bf48a9f3-db73-4f26-b1e7-edac5f3fba32)

## Building

- Windows: [docs/build-windows.md](docs/build-windows.md)
- MacOS: [docs/build-macos.md](docs/build-macos.md)
- Linux: [docs/build-linux.md](docs/build-linux.md)
- Android: [docs/build-android.md](docs/build-android.md)

## Running Tests

`just test` will run all unit and integration tests. To run tests for a specific package, use `just gui/ test` for example.
