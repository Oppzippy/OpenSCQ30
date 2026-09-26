## Building openscq30-cli on macOS

1. Install rust
2. Install the Xcode command line tools (`xcode-select --install`)
3. Run `cargo build --package openscq30-cli --profile release-fast` (or `cargo build --package openscq30-cli --release`, but it's very slow to build)
4. The compiled binary can be found at `target/release-fast/openscq30`

## Building openscq30-gui on macOS

### Just the executable

1. Install rust
2. Install the Xcode command line tools (`xcode-select --install`)
3. Run `cargo build --package openscq30-gui --profile release-fast` (or `cargo build --package openscq30-gui --release`, but it's very slow to build)
4. The compiled binary can be found at `target/release-fast/openscq30-gui`

### App bundle

1. Install rust, the Xcode command line tools, and [just](https://github.com/casey/just)
2. Run `just build-gui-fast` (or `just build-gui`, but it's very slow to build)
3. Run `just build-gui-bundle`
4. The app bundle can be found at `build-output/OpenSCQ30.app`

The app bundle is ad-hoc signed but not notarized. A locally built copy should open normally. A copy downloaded from elsewhere (e.g. a release zip) will still have the quarantine flag set, so macOS will refuse to open it with a plain double click; right click the app and choose "Open" instead, then confirm in the dialog that appears.
