## Building OpenSCQ30 on Linux

Instructions use Ubuntu package names. Package names may differ on other distros.

1. Checkout the repository and its submodules
2. Install rust
3. Install [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
4. Install libdbus-1-dev pkg-config libgtk-4-dev libadwaita-1-dev
5. `cd` to the `gui` directory and run `cargo make --profile release build`. Note that `--profile release` must come before `build`.
6. The compiled binary can be found at `target/release/openscq30_gui`. Locale files are in `target/release/share`.
