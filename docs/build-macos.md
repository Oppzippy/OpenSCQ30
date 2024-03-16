## Building OpenSCQ30 on MacOS

1. Checkout the repository and its submodules
2. Install rust
3. Install [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
4. Install gtk4 and libadwaita (`brew install gtk4 libadwaita`)
5. `cd` to the `gui` directory and run `cargo make --profile release build`. Note that `--profile release` must come before `build`.
6. The compiled binary can be found at `target/release/openscq30_gui`. Locale files are in `target/release/share`.
