## This branch is for v2, which is a work in progress. See the v1 branch.

[v1 branch here](https://github.com/Oppzippy/OpenSCQ30/tree/v1)

## About

OpenSCQ30 is free software for controlling settings for Soundcore headphones and earbuds. It was originally intended for the Q30's, after which the project was named, but a range of devices are now supported.

### Supported Platforms

[x] Windows - Ready  
[x] Linux - Ready  
[x] Android - Ready

### Supported Devices

| Model | Name              | Status |
| ----- | ----------------- | ------ |
| A3004 | Q20i              | Done   |
| A3027 | Life Q35          | Done   |
| A3028 | Life Q30          | Done   |
| A3029 | Life Tune         | Done   |
| A3030 | Life Tune Pro     | Done   |
| A3031 | Vortex            | Done   |
| A3033 | Life 2 Neo        | Done   |
| A3926 | Life Dot 2S       | Done   |
| A3930 | Liberty 2 Pro     | Done   |
| A3931 | Life Dot 2 NC     | Done   |
| A3933 | Life Note 3       | Done   |
| A3935 | Life A2 NC        | Done   |
| A3936 | Space A40         | Done   |
| A3939 | Life P3           | Done   |
| A3945 | Life Note 3S      | Done   |
| A3948 | A20i              | Done   |
| A3951 | Liberty Air 2 Pro | Done   |
| A3959 | R50i NC           | Done   |

### Requested Devices

| Model | Name                    | Issue                                                      |
| ----- | ----------------------- | ---------------------------------------------------------- |
| A3035 | Soundcore Space One     | [GH-140](https://github.com/Oppzippy/OpenSCQ30/issues/140) |
| A3040 | Soundcore Space Q45     | [GH-103](https://github.com/Oppzippy/OpenSCQ30/issues/103) |
| A3116 | Soundcore Motion+       | [GH-119](https://github.com/Oppzippy/OpenSCQ30/issues/119) |
| A3910 | Soundcore Liberty Air 2 | [GH-128](https://github.com/Oppzippy/OpenSCQ30/issues/128) |
| A3947 | Soundcore Liberty 4 NC  | [GH-107](https://github.com/Oppzippy/OpenSCQ30/issues/107) |
| A3949 | Soundcore P20i          | [GH-108](https://github.com/Oppzippy/OpenSCQ30/issues/108) |
| A3952 | Soundcore Liberty 3 Pro | [GH-134](https://github.com/Oppzippy/OpenSCQ30/issues/134) |
| A3954 | Soundcore Liberty 4 Pro | [GH-169](https://github.com/Oppzippy/OpenSCQ30/issues/169) |
| A3955 | Soundcore P40i          | [GH-159](https://github.com/Oppzippy/OpenSCQ30/issues/159) |
| A3959 | Soundcore P30i          | [GH-164](https://github.com/Oppzippy/OpenSCQ30/issues/164) |
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
