## About

OpenSCQ30 is free software for controlling settings for the Soundcore Q30 headphones.

Progress on supported platforms:  
[x] Android - Ready  
[x] Linux - Ready  
[x] Windows - Mostly Ready  
[ ] Mac - It compiles. It's untested beyond that. I don't have a mac, so this probably isn't going anywhere.

## Building

Compiled binaries won't be provided until the first release. Instructions for compiling are available below for [Windows](#windows), [Mac](#mac), [Linux](#linux), and [Android](#android).

### Windows

1. Checkout the repository.
2. Install [gvsbuild](https://github.com/wingtk/gvsbuild) and its dependencies using the [instructions in the readme](https://github.com/wingtk/gvsbuild#development-environment).
3. Follow the [instructions for building GTK4 and libadwaita](https://github.com/wingtk/gvsbuild#build-gtk).
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
3. Install gtk4 and libadwaita (`brew install gtk4 libadwaita`)
4. In the root project directory, run: `cargo build --release --package openscq30_gui`
5. The compiled binary can be found at `target/release/openscq30_gui`

### Linux

Instructions use Ubuntu package names. Package names may differ on other distros.

1. Checkout the repository
2. Install rust
3. Install libdbus-1-dev pkg-config libgtk-4-dev libadwaita-1-dev
4. In the root project directory, run: `cargo build --release --package openscq30_gui`
5. The compiled binary can be found at `target/release/openscq30_gui`

### Android

#### If you do not have Android Studio installed:

1. Install a JDK distribution of your choice
2. Download the [Android command line tools (scroll down a bit)](https://developer.android.com/studio), extract it, and set the `ANDROID_HOME` environment variable to the cmdline-tools directory (the one containing `NOTICE.txt`).
3. Accept licenses by `cd`ing to cmdline-tools/bin and run `./sdkmanager --licenses --sdk_root=..`.
4. Check the version of ndk listed in [`android/app/build.gradle`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle) (Ctrl-F ndkVersion) and [download that version of the ndk](https://developer.android.com/ndk/downloads), extract it, and set the `ANDROID_NDK_HOME` environment variable to the directory inside containing `NOTICE`.

#### If you do have Android Studio installed:

1. Ensure the version of ndk listed in [`android/app/build.gradle`](https://github.com/Oppzippy/OpenSCQ30/blob/master/android/app/build.gradle) (Ctrl-F ndkVersion) is installed (File -> Settings -> Appearance & Behavior -> System Settings -> Android SDK -> SDK Tools).

#### Then:

1. Checkout the repository
2. Install rustup
3. Add all supported cpu architecture targets: `rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android`
4. Install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk): `cargo install cargo-ndk`
5. In the `android` directory, run `./gradlew assembleRelease`
6. The apk can be found at `android/app/build/outputs/apk/release/app-release-unsigned.apk`
