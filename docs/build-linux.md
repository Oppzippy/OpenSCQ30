## Building OpenSCQ30 on Linux

Instructions use Ubuntu package names. Package names may differ on other distros.

1. Install rust
2. Install pkg-config libdbus-1-dev libxkbcommon-dev
3. Run `cargo build --package openscq30-gui --release`
4. The compiled binary can be found at `target/release/openscq30-gui`

## Runtime Dependencies
- [cosmic-icons](https://github.com/pop-os/cosmic-icons/): if a package isn't available, clone the git repo and run `just install`.
