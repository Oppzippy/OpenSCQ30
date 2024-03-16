## Building OpenSCQ30 on MacOS

1. Install rust
2. Install [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
3. Install gtk4 and libadwaita (`brew install gtk4 libadwaita`)
4. `cd` to the `gui` directory and run `cargo make --profile release build`. Note that `--profile release` must come before `build`.
5. The compiled binary can be found at `target/release/openscq30_gui`. Locale files are in `target/release/share`.
