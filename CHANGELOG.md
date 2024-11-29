# Changelog

## v1.14.1

### General

#### Fixes

-   Fix Space A40 (A3936) state update packet parsing

## v1.14.0

### General

#### Features

-   Add support for Space A40 (A3936)
-   Add support for Vortex (A3031)

#### Fixes

-   Fix Liberty 2 Pro (A3930) packet parse error

### GUI

#### Fixes

-   Quick presets resetting to default values
-   Sound mode options unsupported by the device visible in quick presets

### Android

#### Fixes

-   Rename General menu to Sound Modes

## v1.13.1

### GUI

#### Fixes

-   Remove broken gtk binding that was ignored and had no effect other than a critical log message

## v1.13.0

### General

#### Build

-   Replace `cargo-make` with `just`

### GUI

#### Features

-   Add custom equalizer profile importing/exporting

#### Fixes

-   Fix some circumstances where the volume adjustments matching a custom profile would be selected, but the dropdown would not show the profile as selected
-   Custom equalizer profile sorting is now case insensitive
-   Icon colors now adapt when using dark theme
-   Write config file to temp file first before overwriting to work around potential data loss if writing is interrupted
-   The config file is no longer overwritten with the exact same content during every application startup

### Android

#### Features

-   Add custom equalizer profile importing/exporting
-   Show the title of the current screen rather than the device's name in the header bar
-   New color scheme generated from logo color

#### Fixes

-   To resolve inconsistent transition animations, use slides everywhere
-   Custom equalizer profile duplicate detection issues

### Web

#### Features

-   Add custom equalizer profile importing/exporting
-   There is now a dropdown menu to select light/dark mode in addition to the default option of following the system preference

#### Fixes

-   Accesibility issues

## v1.12.0

### CLI

#### Features

-   Add shell completions: `openscq30_cli completions`

### Android

#### Features

-   Add option to auto connect to paired devices  
    In order to implement this, the companion device API is now used, which means connected devices will no longer be listed by default. They must first be paired with the app. This was necessary to gain permission to start foreground services from the background, which is necessary for auto connect to work.

## v1.11.0

### Android

#### Features

-   Add custom button settings
-   Reorganize UI to allow for more than 5 total settings pages

### Web

#### Features

-   Add custom button settings
-   Improve layout on larger screens

## v1.10.6

### General

#### Fixes

-   Feature not supported error when changing ambient sound mode cycle even when the feature is supported

### GUI

#### Fixes

-   Custom noise canceling option sometimes not visible when it should be, and sometimes visible when it shouldn't be
-   Rename button action "Trans" to "Ambient Sound Mode", since that's what it does, cycles through ambient sound modes
-   Devices other than Q30 potentially not working on Linux

## v1.10.5

### General

#### Fixes

-   Fall back to default values when device is in an invalid state rather than failing to connect

## v1.10.4

### General

#### Fixes

-   Fix A3933 and A3939 state update packet parsing

## v1.10.3

### Web

#### Fixes

-   Device profile table cell overflowing, breaking layout

## v1.10.2

### Android

#### Fixes

-   Various resource leaks when failing to connect to a device
-   Edge case where events occur in unexpected order while connecting, causing it to hang

## v1.10.1

### General

#### Fixes

-   A3933 equalizer not working

### Android

#### Fixes

-   Immediate crash due to misconfigured proguard

## v1.10.0

### General

#### Features

-   Support for A3945 (Note 3S), A3933 (Note 3), and A3939 (Life P3)

## v1.9.0

### General

#### Features

-   Partial support for A3945, A3933, and A3939. Not ready for general use.

### CLI

#### Fixes

-   Panic on exit due to nested tokio runtimes
-   "get equalizer" returned values in the range of 0 to 255 rather than -120 to 135
-   "set equalizer" accepted values in the range of -12 to 13 rather than -120 to 135

### Web

#### Fixes

-   Noise Canceling showing on devices that should not have that option

### Android

#### Fixes

-   Custom profiles sometimes not showing as selected despite the equalizer being set correctly

## v1.8.1

### Android

#### Fixes

-   Add a few retries and a timeout when connecting to a device

## v1.8.0

### GUI

#### Features

-   Add quick presets
-   Add button keybinding settings
-   Add hear id settings

#### Fixes

-   Localize equalizer preset names

#### Dependencies

-   Minimum gtk4 version increased to v4.12 (GNOME 45)
-   Minimum libadwaita version increased to v1.4 (GNOME 45)

## v1.7.0

### GUI

#### Fixes

-   Bluetooth initialization when launching the application will no longer block the window from showing

### Android

#### Features

-   If you have more than one type of Soundcore device, they will now each have their own sets of quick presets. To avoid having this be a breaking change, any newly connected devices' quick presets will be initialized with your quick preset configuration prior to this release.

#### Fixes

-   Fix settings not used by the connected device showing in quick presets
-   Fix screen rotation causing disconnect
-   Refresh device list when granting bluetooth permission

## v1.6.0

### General

#### Features

-   Add experimental support for new devices: A3027, A3028, A3030, A3033, A3033EU, A3926, A3926Z11, A3930, A3931, A3931XR, A3935, A3935W, A3951
-   Add support for custom transparency mode
-   Add support for custom noise canceling mode
-   Add device information tab

### Android

#### Fixes

-   Fix notification buttons not working on API level 34 (Android 14)

### Web

#### Features

-   Add loading indicator while in the process of connecting to device

## v1.5.2

### GUI

#### Fixes

-   Fix non-soundcore devices showing in device list

### Android

#### Fixes

-   Pull to refresh icon now follows color scheme
-   No longer crashes on api level 34 (android 14 beta)

### Web

#### Fixes

-   Disallow deselecting sound mode buttons

## v1.5.1

### Android

#### Fixes

-   Fix noise canceling mode displaying in notification as $1Noise Canceling
-   Fix text fields not being single line
-   Fix equalizer remaining checked when moving between Quick Preset tabs

## v1.5.0

### GUI

#### Fixes

-   Go back to device selection screen immediately when device disconnects

### Android

#### Features

-   Added "Quick Presets", which enable changing ambient sound mode, noise canceling mode, and equalizer profile all together with the press of a notification button.

## v1.4.0

### General

#### Fixes

-   Improve handing of device disconnects. How this is implemented varies by platform. The desktop GUI could still use some more work.

### Android

-   Minimum Android API level increased from 24 to 26 (Android 7.0 to Android 8.0)

#### Features

-   The bluetooth connection is now held by a service, so it can be kept open in the background. This is in preparation for having sound mode and eq profile buttons in a notification.

## v1.3.1

### GUI

#### Fixes

-   Fix arm64 builds

## v1.3.0

### General

#### Features

-   Allow modifying equalizer directly from a preset profile rather than having to switch the dropdown to Custom first
-   Allow selecting a custom profile while a preset profile is selected rather than having to select Custom first

### Web

#### Fixes

-   Fixed custom profile dropdown having incorrect aria label

## v1.2.0

### General

#### Features

-   Add web client (requires Web Bluetooth, currently only supported by Chromium)
-   Add equalizer visualization for all profile dropdowns
-   Only show create custom profile button when a custom profile is not selected, and only show delete custom profile button when one is selected

#### Fixes

-   Fix some Soundcore devices not being detected due to a previously unknown mac address prefix

### GUI

#### Features

-   Add volume text input in addition to slider

### Android

#### Features

-   Add dialog for replacing existing custom profiles without retyping the name

## v1.1.1

### Android

#### Fixes

-   Fixed equalizer number input range being smaller than slider range

## v1.1.0

### General

#### Features

-   Equalizer range increased from -6db-+6db to -12db-+12db

### GUI

#### Features

-   Add light/dark mode support on Windows

#### Fixes

-   Increased default height of window

## v1.0.0

Initial release
