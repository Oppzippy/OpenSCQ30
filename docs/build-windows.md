## Building OpenSCQ30 on Windows

### Just the executable

1. Install rust
2. Run `cargo build --package openscq30-gui --release`
3. The compiled binary can be found at `.\target\release\openscq30-gui.exe`

### Installer

1. Follow the instructions for building just the executable
2. Install [Inno Setup](https://jrsoftware.org/isinfo.php)
3. Run `./packaging/windows/build.sh` in git bash or similar. It must be an environment that has access to Inno Setup, so no WSL.
4. The installer can be found at `.\packaging\windows\Output\OpenSCQ30-Setup.exe`
