## Building OpenSCQ30 on MacOS

1. Install rust
2. Install [Just](https://just.systems/man/en/chapter_4.html): `brew install just`
3. Install gtk4 and libadwaita: `brew install gtk4 libadwaita`
4. Run `just gui/ build release`
5. The compiled binary can be found at `target/release/openscq30_gui`. Locale files are in `gui/share`.
