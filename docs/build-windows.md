## Building OpenSCQ30 on Windows

1. Install MSYS2 and run the UCRT64 environment.
2. Install dependencies using `pacman -Syu base-devel mingw-w64-ucrt-x86_64-libadwaita mingw-w64-ucrt-x86_64-rust mingw-w64-ucrt-x86_64-pkg-config`.
3. Install [Just](https://just.systems/man/en/chapter_4.html): `cargo install just`
4. Run `./packaging/windows/build.sh`
5. The compiled binary along with all dependencies can be found at `packaging/windows/root`. This can then be moved and run from outside of the MSYS2 environment.
