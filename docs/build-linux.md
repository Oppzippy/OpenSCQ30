Instructions use Ubuntu package names. Package names may differ on other distros.

If it's inconvenient to install the latest version of [just](https://github.com/casey/just), use the without just instructions. The catch is that the without just instructions are more likely to change in the future, so if you're packaging openscq30 and the latest version of just is easily available, prefer the with just instructions.

## Building openscq30-cli on Linux

1. Install the latest version of rust

### Without just

2. Run `cargo build --package openscq30-cli --profile release-fast` (or `cargo build --package openscq30-cli --release`, but it's very slow to build)
3. The compiled binary can be found at `target/release-fast/openscq30`

### With just

2. Run `just build-cli-fast` (or `just build-cli` but it's very slow to build)
3. The compiled binary can be found at `build-output/openscq30`

## Building openscq30-gui on Linux

1. Install the latest version of rust
2. Install pkg-config libdbus-1-dev libxkbcommon-dev

### Without just

3. Run `cargo build --package openscq30-gui --profile release-fast` (or `cargo build --package openscq30-gui --release`, but it's very slow to build)
4. The compiled binary can be found at `target/release-fast/openscq30-gui`

### With just

3. Run `just build-gui-fast` (or `just build-gui` but it's very slow to build)
4. The compiled binary can be found at `build-output/openscq30-gui`

## Runtime Dependencies

- [cosmic-icons](https://github.com/pop-os/cosmic-icons/): if a package isn't available, clone the git repo and run `just install`.
