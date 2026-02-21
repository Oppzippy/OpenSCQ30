## About

OpenSCQ30 is free software for controlling settings for Soundcore headphones and earbuds. It was originally intended for the Soundcore Life Q30, after which the project was named, but a range of devices are now supported.

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
| A3035 | Soundcore Space One                |
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
| A3949 | Soundcore P20i / P25i / R50i       |
| A3951 | Soundcore Liberty Air 2 Pro        |
| A3955 | Soundcore P40i                     |
| A3957 | Soundcore Liberty 5                |
| A3959 | Soundcore P30i / Soundcore R50i NC |

## Installing

See [GitHub Releases](https://github.com/Oppzippy/OpenSCQ30/releases). All files are signed with [my GPG key](https://kylescheuing.com/publickey.txt).

[![Flathub](https://img.shields.io/flathub/v/com.oppzippy.OpenSCQ30)](https://flathub.org/apps/com.oppzippy.OpenSCQ30)
[![IzzyOnDroid](https://img.shields.io/endpoint?url=https://apt.izzysoft.de/fdroid/api/v1/shield/com.oppzippy.openscq30)](https://apt.izzysoft.de/fdroid/index/apk/com.oppzippy.openscq30)

[![Packaging status](https://repology.org/badge/vertical-allrepos/openscq30.svg)](https://repology.org/project/openscq30/versions)

## Mirrors

Issues and pull requests are accepted on both GitHub and Codeberg, although CI and releases are only on github.

- GitHub: https://github.com/Oppzippy/OpenSCQ30
- Codeberg: https://codeberg.org/Oppzippy/OpenSCQ30

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
