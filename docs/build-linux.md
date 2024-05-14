## Building OpenSCQ30 on Linux

Instructions use Ubuntu package names. Package names may differ on other distros.

1. Install rust
2. Install [Just](https://just.systems/man/en/chapter_4.html)
3. Install libdbus-1-dev pkg-config libgtk-4-dev libadwaita-1-dev
4. Run `just gui/ build release`
5. The compiled binary can be found at `target/release/openscq30_gui`. Locale files are in `gui/share`.
