## About

OpenSCQ30 is free software for controlling settings for the Soundcore Q30 headphones.

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
| A3030    | Life Tune Pro     | Unknown |
| A3033    | Life 2 Neo        | Unknown |
| A3033EU  | Life 2 Neo        | Unknown |
| A3926    | Life Dot 2S       | Unknown |
| A3926Z11 | Life Dot 2S       | Unknown |
| A3930    | Liberty 2 Pro     | Unknown |
| A3931    | Life Dot 2 NC     | Unknown |
| A3931XR  | Life Dot 2 XR     | Unknown |
| A3935    | Life A2 NC        | Unknown |
| A3935W   | Life A2 NC        | Unknown |
| A3951    | Liberty Air 2 Pro | Unknown |
| A3945    | Life Note 3S      | Unknown |
| A3933    | Life Note 3       | Unknown |
| A3939    | Life P3           | Working |

## Installing

[![Flathub](https://img.shields.io/flathub/v/com.oppzippy.OpenSCQ30)](https://flathub.org/apps/com.oppzippy.OpenSCQ30)
[![](https://img.shields.io/endpoint?url=https://apt.izzysoft.de/fdroid/api/v1/shield/com.oppzippy.openscq30)](https://apt.izzysoft.de/fdroid/index/apk/com.oppzippy.openscq30)

See [GitHub Releases](https://github.com/Oppzippy/OpenSCQ30/releases). All files are signed with [my GPG key](https://kylescheuing.com/publickey.txt).

## Demo

### Desktop

[Desktop GUI Demo.webm](https://user-images.githubusercontent.com/2236514/229958756-aaa2a6d3-e908-4195-aad6-b0bcfda139a8.webm)

### Android

[android-demo.webm](https://github.com/Oppzippy/OpenSCQ30/assets/2236514/2d351d63-64b8-4253-abdf-3bb5384888c1)

## Building

### Windows

1. Checkout the repository and its submodules
2. Install [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
3. Install [gvsbuild](https://github.com/wingtk/gvsbuild) and its dependencies using the [instructions in the readme](https://github.com/wingtk/gvsbuild#development-environment).
4. Follow the [instructions for building GTK4 and libadwaita](https://github.com/wingtk/gvsbuild#build-gtk).
5. Set the [environment variables from the gvsbuild instructions](https://github.com/wingtk/gvsbuild#add-gtk-to-your-environmental-variables)
6. `cd` to the `gui` directory and run `cargo make --profile release build`. Note that `--profile release` must come before `build`.
7. The compiled binary can be found at `target\release\openscq30_gui.exe`
8. For distribution, make a new folder and copy the following into it:

| From                                                | To                     |
| --------------------------------------------------- | ---------------------- |
| target\release\openscq30_gui.exe                    | bin\openscq30_gui.exe  |
| target\release\share                                | share                  |
| C:\gtk-build\gtk\x64\release\bin\\\*.dll            | bin\\\*.dll            |
| C:\gtk-build\gtk\x64\release\bin\gdbus.exe          | bin\gdbus.exe          |
| C:\gtk-build\gtk\x64\release\share\glib-2.0\schemas | share\glib-2.0\schemas |
| C:\gtk-build\gtk\x64\release\share\locale           | share\locale           |

### Mac

1. Checkout the repository and its submodules
2. Install rust
3. Install [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
4. Install gtk4 and libadwaita (`brew install gtk4 libadwaita`)
5. `cd` to the `gui` directory and run `cargo make --profile release build`. Note that `--profile release` must come before `build`.
6. The compiled binary can be found at `target/release/openscq30_gui`. Locale files are in `target/release/share`.

### Linux

Instructions use Ubuntu package names. Package names may differ on other distros.

1. Checkout the repository and its submodules
2. Install rust
3. Install [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
4. Install libdbus-1-dev pkg-config libgtk-4-dev libadwaita-1-dev
5. `cd` to the `gui` directory and run `cargo make --profile release build`. Note that `--profile release` must come before `build`.
6. The compiled binary can be found at `target/release/openscq30_gui`. Locale files are in `target/release/share`.

### Android

#### If you do not have Android Studio installed:

1. Install a JDK distribution of your choice
2. Download the [Android command line tools (scroll down a bit)](https://developer.android.com/studio), extract it, and set the `ANDROID_HOME` environment variable to the cmdline-tools directory (the one containing `NOTICE.txt`).
3. Accept licenses by `cd`ing to cmdline-tools/bin and run `./sdkmanager --licenses --sdk_root=..`.
4. Check the version of ndk listed in [`android/app/build.gradle`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle) (Ctrl-F ndkVersion) and [download that version of the ndk](https://developer.android.com/ndk/downloads), extract it, and set the `ANDROID_NDK_HOME` environment variable to the directory inside containing `NOTICE`.

#### If you do have Android Studio installed:

1. Ensure the version of ndk listed in [`android/app/build.gradle`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle) (Ctrl-F ndkVersion) is installed (File -> Settings -> Appearance & Behavior -> System Settings -> Android SDK -> SDK Tools).

#### Then:

1. Checkout the repository and its submodules
2. Install rustup
3. Add all supported cpu architecture targets: `rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android`
4. Install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk): `cargo install cargo-ndk`
5. In the `android` directory, run `./gradlew assembleRelease`
6. The apk can be found at `android/app/build/outputs/apk/release/app-release-unsigned.apk`

## Running Tests

`cargo make test` will run all unit and integration tests.
