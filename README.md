## About

OpenSCQ30 is free software for controlling settings for the Soundcore Q30 headphones.

Progress on supported platforms:  
[x] Android - Ready  
[x] Linux - Ready  
[ ] Windows - It works in demo mode, but probably doesn't work with a real device.  
[ ] Mac - It compiles. It's untested beyond that. I don't have a mac, so this probably isn't going anywhere.

## Building

Compiled binaries won't be provided until the first release. Instructions for compiling are available below for [Windows](#windows), [Mac](#mac), [Linux](#linux), and [Android](#android).

### Windows

1. Checkout the repository.
2. Install [gvsbuild](https://github.com/wingtk/gvsbuild) and its dependencies using the [instructions in the readme](https://github.com/wingtk/gvsbuild#development-environment).
3. Follow the [instructions for building GTK4](https://github.com/wingtk/gvsbuild#build-gtk).
4. Set the [environment variables from the gvsbuild instructions](https://github.com/wingtk/gvsbuild#add-gtk-to-your-environmental-variables) and run `cargo build --release --package openscq30_gui`.
5. The compiled binary can be found at `target\release\openscq30_gui.exe`
6. For distribution, make a new folder and copy the following into it:

-   target\release\openscq30_gui.exe
-   C:\gtk-build\gtk\x64\release\bin\\\*.dll
-   C:\gtk-build\gtk\x64\release\bin\gdbus.exe

7. In the new folder, make a `share` folder, and a `glib-2.0` folder inside of that. Then copy `C:\gtk-build\gtk\x64\release\share\glib-2.0\schemas` into the newly created `glib-2.0` folder.

### Mac

1. Checkout the repository
2. Install rust
3. Install gtk4 (`brew install gtk4`)
4. Install libadwaita (`brew install libadwaita`) (optional)
5. In the root project directory, run: `cargo build --release --package openscq30_gui`. To build with libadwaita, add `--features libadwaita`.
6. The compiled binary can be found at `target/release/openscq30_gui`

### Linux

Instructions use Ubuntu package names. Package names may differ on other distros.

1. Checkout the repository
2. Install rust
3. Install libdbus-1-dev pkg-config libgtk-4-dev
4. Install libadwaita-1-dev (optional)
5. In the root project directory, run: `cargo build --release --package openscq30_gui`. To build with libadwaita, add `--features libadwaita`.
6. The compiled binary can be found at `target/release/openscq30_gui`

### Android

1. Checkout the repository
2. Install rustup
3. Add all supported cpu architecture targets: `rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android`
4. Install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk): `cargo install cargo-ndk`
5. If you have Android Studio installed, ensure the version of ndk listed in [android/app/build.gradle](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle) (ctrl+f ndkVersion) is installed. If you don't have Android Studio installed, skip this step.
6. If you don't have Android Studio installed, [download the ndk](https://developer.android.com/ndk/downloads) and [follow the instructions on setting `ANDROID_NDK_HOME` from cargo-ndk](https://github.com/bbqsrc/cargo-ndk#usage).
7. In the `android` directory, run `./gradlew assembleRelease`
8. The apk can be found at `android/app/build/outputs/apk/release/app-release-unsigned.apk`
