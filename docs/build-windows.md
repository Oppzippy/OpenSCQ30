## Building OpenSCQ30 on Windows

1. Install [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
2. Install [gvsbuild](https://github.com/wingtk/gvsbuild) and its dependencies using the [instructions in the readme](https://github.com/wingtk/gvsbuild#development-environment).
3. Follow the [instructions for building GTK4 and libadwaita](https://github.com/wingtk/gvsbuild#build-gtk).
4. Set the [environment variables from the gvsbuild instructions](https://github.com/wingtk/gvsbuild#add-gtk-to-your-environmental-variables)
5. `cd` to the `gui` directory and run `cargo make --profile release build`. Note that `--profile release` must come before `build`.
6. The compiled binary can be found at `target\release\openscq30_gui.exe`
7. For distribution, make a new folder and copy the following into it:

| From                                                | To                     |
| --------------------------------------------------- | ---------------------- |
| target\release\openscq30_gui.exe                    | bin\openscq30_gui.exe  |
| target\release\share                                | share                  |
| C:\gtk-build\gtk\x64\release\bin\\\*.dll            | bin\\\*.dll            |
| C:\gtk-build\gtk\x64\release\bin\gdbus.exe          | bin\gdbus.exe          |
| C:\gtk-build\gtk\x64\release\share\glib-2.0\schemas | share\glib-2.0\schemas |
| C:\gtk-build\gtk\x64\release\share\locale           | share\locale           |
