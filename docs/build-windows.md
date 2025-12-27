## Building openscq30-cli on Windows

1. Install rust
2. Run `cargo build --package openscq30-cli --profile release-fast` (or `cargo build --package openscq30-cli --release`, but it's very slow to build)
3. The compiled binary can be found at `.\target\release-fast\openscq30.exe`

## Building openscq30-gui on Windows

### Just the executable

1. Install rust
2. Run `cargo build --package openscq30-gui --profile release-fast` (or `cargo build --package openscq30-gui --release`, but it's very slow to build)
3. The compiled binary can be found at `.\target\release-fast\openscq30-gui.exe`

### Installer

1. Install rust, [just](https://github.com/casey/just), and [Inno Setup](https://jrsoftware.org/isinfo.php)
2. Run `just build-gui-fast` (or `just build-gui`, but it's very slow to build) in bash (git bash, msys2, etc.)
3. Run `just build-gui-installer` in bash (git bash, msys2, etc.)
4. The installer can be found at `build-output/openscq30-gui-installer-windows-x86_64.exe`
