## About
OpenSCQ30 is free software for controlling settings for the Soundcore Q30 headphones.

Progress on supported platforms:  
[x] Android - Works but missing some features  
[x] Linux - Works but missing some features  
[ ] Windows - It compiles and doesn't crash on startup. It's untested beyond that.  
[ ] Mac - It compiles. It's untested beyond that.  

## Building  
Compiled binaries won't be provided until the first release. Instructions for compiling are available below for [Windows](#windows), [Mac](#mac), [Linux](#linux), and [Android](#android).

### Windows  
#### Option 1: msys2
1. Install [msys2](https://www.msys2.org/)
2. In msys2, install the following packages:
- git
- mingw-w64-x86_64-gtk4
- mingw-w64-x86_64-pkgconf
- mingw-w64-x86_64-gcc
- mingw-w64-x86_64-rust
- mingw-w64-x86_64-libadwaita (optional)
3. Checkout the repository in msys2
4. In the root project directory, run: `cargo build --release --package openscq30_gui`. To build without libadwaita, run `cargo build --release --package openscq30_gui --no-default-features` instead.
5. The compiled binary can be found at `target/release/openscq30_gui`
6. For distribution, copy the necessary DLLs listed in `.github/workflows/build.yml`

#### Option 2: Building GTK with msvc
TODO


### Mac  
1. Checkout the repository
2. Install rust
3. Install gtk4 (`brew install gtk4`)
4. Install libadwaita (`brew install libadwaita`) (optional)
5. In the root project directory, run: `cargo build --release --package openscq30_gui`. To build without libadwaita, run `cargo build --release --package openscq30_gui --no-default-features` instead.
6. The compiled binary can be found at `target/release/openscq30_gui`

### Linux  
Instructions use Ubuntu package names. Package names may differ on other distros.
1. Checkout the repository
2. Install rust
3. Install libdbus-1-dev pkg-config libgtk-4-dev 
4. Install libadwaita-1-dev (optional)
5. In the root project directory, run: `cargo build --release --package openscq30_gui`. To build without libadwaita, run `cargo build --release --package openscq30_gui --no-default-features` instead.
6. The compiled binary can be found at `target/release/openscq30_gui`

### Android  
1. Checkout the repository
2. Install rust
3. Add all supported cpu architecture targets: `rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android`
4. In the `android` directory, run `./gradlew assembleRelease`
5. The apk can be found at `android/app/build/outputs/apk/release/app-release-unsigned.apk`
