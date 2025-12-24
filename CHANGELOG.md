# Changelog

## Unreleased

### General

#### Fixes

- Changing from transparency mode to normal mode with the Soundcore Space A40 no longer makes unnecessary intermediate steps, selecting noise canceling along the way

### CLI

#### Features

- Add proper --help text with examples
- Add `openscq30 list-models` command to list supported device models (and importantly, their names, so that one can discover that Soundcore Life Q30's device model is SoundcoreA3028 for example). This makes `openscq30 paired-devices add` usable.

#### Fixes

- Fix `openscq30 device list-settings`'s `--no-categories` flag having opposite effect when `--json` is used
- Crash in `openscq30 device list-settings` when a setting is not present (such as when firmware is old and doesn't support a feature)

## v2.0.1

### General

#### Fixes

- Soundcore Space A40 case battery should be out of 10
- Soundcore Space A40 sound modes not working

### Android

#### Packaging

- Separate building for different cpu architectures into their own gradle tasks so when building for a specific one, you don't have to build the rust code for all.
- Fix timestamps of locale files breaking reproducible builds

## v2.0.0

This includes all changes since v1.19.3. For those that have been using the beta versions, there have been no notable changes since v2.0.0-beta7.

### General

#### Breaking Changes

- Custom equalizer profiles are now device specific. Legacy equalizer profiles can be migrated after connecting to a device.
- Quick presets are now stored in a different format, and no automatic migration is available, so they must be re-created.
- Device model auto detection is removed, so you now need to select the device model when connecting.

#### Features

- All clients now share the same database format, so it is possible to share the sqlite file amongst them.
- Connecting to a demo device is now a runtime option rather than compile time. As an exmaple of what this can be used for, exporting a custom equalizer profile without physically having your device with you is now possible.
- Quick presets now include any setting available for the device rather than having to add support for each setting individually.
- Add support for new devices
    - Soundcore Q20i
    - Soundcore A20i
    - Soundcore R50i
    - Soundcore Liberty 4 NC
    - Soundcore Space Q45
    - Soundcore Motion+
    - Soundcore P30i
- Show case battery level for relevant devices
- Add toggles for Gaming Mode, Sound Leak Compensation, Surround Sound, Auto Play/Pause, Wearing Tone, Touch Lock, Low Battery Prompt, and Wearing Detection

#### Localization

- Add German translations (thanks to Ireozar)
- Add Turkish translations (thanks to ozer4 and Ferry7466)
- Add Japanese translations

### GUI

#### Breaking Changes

- To migrate legacy equalizer profiles, see the Legacy Equalizer Profile Migration tab after connecting to a device.
- Rewrite using [libcosmic](https://github.com/pop-os/libcosmic)
- Executable renamed from openscq30_gui to openscq30-gui

#### Features

- Add settings page with option to change preferred language. Use this if you want OpenSCQ30 to be in a different language than the one your operating system is set to.

#### Packaging Changes

- GTK4 and libadwaita are no longer required
- [cosmic-icons](https://github.com/pop-os/cosmic-icons/) is required on Linux

#### Fixes

- On Windows, a terminal window will no longer be shown unless openscq30-gui is launched from the terminal. This change is made to match behavior on Linux.

### CLI

#### Breaking Changes

- All commands have changed, so any scripts making use of the CLI will need to be updated
- Executable renamed from openscq30_cli to openscq30

#### Features

- Add support for custom equalizer profiles
- It is now possible to set/get multiple settings all in one go, rather than having to invoke openscq30_cli multiple times. This improves performance by only connecting once rather than once per get/set.
- Flag for JSON output

### Android

#### Breaking Changes

- To migrate legacy equalizer profiles, see the Legacy Equalizer Profile Migration menu after connecting to a device.

#### Features

- Add support for [per-app language preferences](https://developer.android.com/guide/topics/resources/app-languages)

#### Packaging

- Protobuf is no longer used, so `protoc` is no longer needed when building
- The APK is now split by ABI to reduce file size (in particular, to avoid coming close to the IzzyOnDroid 30MB file size limit). The universal APK is still available, however.

### Web

- Removed web application due to it being impossible to support some devices on this platform. The v1 branch will continue to be available at the same URL for the foreseeable future.

## v2.0.0-beta7

### General

#### Features

- Add support for Soundcore Motion+
- Add support for Soundcore P30i
- Show battery level as percentage

#### Fixes

- Soundcore R50i NC should not have transparency modes
- Soundcore R50i NC had manual and automatic nosie canceling mixed up, causing various issues
- Gaming mode not working on devices other than Soundcore Liberty 4 NC

#### Localization

- Update Turkish translations (thanks to ozer4 and Ferry7466)
- Update German translations (thanks to Ireozar)
- Update Japanese translations

### GUI

#### Features

- Add settings page with option to change preferred language. Use this if you want OpenSCQ30 to be in a different language than the one your operating system is set to.

### CLI

#### Features

- Show battery level as fraction (for example, 2/5 instead of 2)

### Android

#### Features

- Add support for [per-app language preferences](https://developer.android.com/guide/topics/resources/app-languages)

## v2.0.0-beta6

### General

#### Features

- Show case battery level for Life Note 3, Space A40, Life Note 3S, and Liberty 4 NC
- Add gaming mode toggle for Life Note 3, Space A40, Life Note 3S, Liberty 4 NC, and R50i NC
- Add sound leak compensation toggle for Liberty 4 NC
- Add surround sound toggle for Liberty 4 NC
- Add auto play/pause toggle for Liberty 4 NC
- Add wearing tone toggle for Liberty 4 NC
- Add touch lock toggle for Liberty 4 NC
- Add low battery prompt toggle for Liberty 4 NC and R50i NC
- Add wearing detection toggle for Life Q35, Life 2 Neo, Life Note 3, Life Note 3S, and Liberty Air 2 Pro
- Add battery level for Space Q45
- Add button configuration for Space Q45

#### Localization

- Add more Turkish translations (thanks to ozer4)

## v2.0.0-beta5

### General

#### Features

- Add support for Soundcore Liberty 4 NC
- Add support for Soundcore Space Q45

#### Fixes

- Disable HearID when modifying the equalizer, since HearID is not shown in the UI and it overrides the equalizer
- Soundcore A20i incorrectly displaying buttons as disabled in some situations
- Soundcore Life Note 3s button configuration displaying incorrect button actions

#### Localization

- Add German translations (thanks to Ireozar)
- Add Turkish translations (thanks to ozer4)

## v2.0.0-beta4

### Android

#### Fixes

- Crash when connecting to a disconnected device
- Crash when viewing device information tab for Soundcore Development Information

## v2.0.0-beta3

### General

- Add Codeberg mirror: https://codeberg.org/Oppzippy/OpenSCQ30

#### Features

- Add Reset Buttons to Default action

#### Fixes

- Button configuration not working for various devices
- None option shown for buttons that can't have their actions disabled
- Equalizer settings shown when only one earbud is connected
- Soundcore R50i NC missing TWS status in device information category

### GUI

#### Fixes

- Connecting to a powered off device, canceling before it times out, and connecting to a second powered on device will no longer disconnect from the second device when the connection to the first times out.

#### Localization

- Weblate is now used for translations using Codeberg's instance: https://translate.codeberg.org/projects/openscq30/

## v2.0.0-beta2

### General

#### Fixes

- R50i NC sound modes not working
- Life Q30 not working with firmware version 05.19

### GUI

#### Fixes

- Missing search icon on windows

## v2.0.0-beta1

### General

#### Breaking Changes

- Custom equalizer profiles are now device specific. Legacy equalizer profiles can be migrated after connecting to a device.
- Quick presets are now stored in a different format, and no automatic migration is available, so they must be re-created [1].
- Device model auto detection is removed, so you now need to select the device model when connecting [2].

[1] For anyone curious about the reason that automatic migration is impractical: Quick presets were made device specific in v1 by using information only retrievable over BLE, but I wanted to entirely move over to RFCOMM only for v2, so it is now not possible to determine which quick preset goes with which device.

[2] This is a minor inconvenience for the user, but makes it easier to potentially add support for devices from other manufacturers in the future.

#### Features

- All clients now share the same database format, so it is possible to share the sqlite file amongst them.
- Connecting to a demo device is now a runtime option rather than compile time. As an exmaple of what this can be used for, exporting a custom equalizer profile without physically having your device with you is now possible.
- Quick presets now include any setting available for the device rather than having to add support for each setting individually.
- Add support for new devices
    - Soundcore Q20i
    - Soundcore A20i
    - Soundcore R50i

### GUI

#### Breaking Changes

- To migrate legacy equalizer profiles, see the Legacy Equalizer Profile Migration tab after connecting to a device.
- Rewrite using [libcosmic](https://github.com/pop-os/libcosmic)
- Executable renamed from openscq30_gui to openscq30-gui

#### Packaging Changes

- GTK4 and libadwaita are no longer required
- [cosmic-icons](https://github.com/pop-os/cosmic-icons/) is required on Linux

#### Fixes

- On Windows, a terminal window will no longer be shown. This change is made to match behavior on Linux.

### CLI

#### Breaking Changes

- All commands have changed, so any scripts making use of the CLI will need to be updated
- Executable renamed from openscq30_cli to openscq30

#### Features

- Add support for custom equalizer profiles
- It is now possible to set/get multiple settings all in one go, rather than having to invoke openscq30_cli multiple times. This improves performance by only connecting once rather than once per get/set.
- Flag for JSON output

### Android

#### Breaking Changes

- To migrate legacy equalizer profiles, see the Legacy Equalizer Profile Migration menu after connecting to a device.

### Web

- Removed web application due to it being impossible to support some devices on this platform

## v1.19.3

### General

#### Fixes

- Add missing mac address prefix 88:0E:85

## v1.19.2

### General

#### Fixes

- A3936 packet parsing issues
- Connection issues on Windows when a device is paired with multiple bluetooth adapters

## v1.19.1

### General

#### Fixes

- Fix newer Life Q30's failing to connect due to parsing error

## v1.19.0

### Android

#### Features

- Add button in settings to copy logs to clipboard for bug reports

## v1.18.1

### General

#### Fixes

- Fix serial number not being retrieved for some devices, leading to quick presets not working

## v1.18.0

### General

#### Fixes

- Incorrect button action implementation for all devices. Setting actions somewhat worked, but existing actions were parsed incorrectly.
- Life Note 3 (A3933) sound mode changes by pressing a button on the device not being handled
- Space A40 (A3936) button action packet serialization mistake
- State update packet parse error when the right earbud is the TWS host device

### GUI

#### Fixes

- Connect using BR/EDR instead of BLE. All Soundcore devices support BR/EDR, but only some support BLE.

### Android

#### Features

- Improve pairing process. Scanning is no longer necessary.
- Add setting to disable mac address filter for device listing.

#### Fixes

- Connect using BR/EDR instead of BLE. All Soundcore devices support BR/EDR, but only some support BLE.

## v1.17.0

### Android

#### Features

- Added button for scanning for devices without any filters. Useful for devices with names not containing "Soundcore".

### Web

#### Features

- Added button for scanning for devices without any filters. Useful for devices with new mac address ranges.

## v1.16.1

### Android

#### Fixes

- Fix devices with names containing "soundcore" (lowercase s) rather than "Soundcore" not showing up in the listing.

## v1.16.0

### General

#### Features

- Add Game Mode button action

#### Fixes

- Fix service not found error on Linux unless `bluetoothctl connect` was manually run

## v1.15.0

### General

#### Fixes

- Add missing soundcore mac address ranges for device detection
- Prioritize filtering by GATT service id rather than mac address, since new mac address ranges are showing up. This isn't possible (or implemented yet) on every client, however.
- Various Space A40 issues that prevented it from working at all

### CLI

#### Features

- Add support for all existing sound modes

## v1.14.1

### General

#### Fixes

- Fix Space A40 (A3936) state update packet parsing

## v1.14.0

### General

#### Features

- Add support for Space A40 (A3936)
- Add support for Vortex (A3031)

#### Fixes

- Fix Liberty 2 Pro (A3930) packet parse error

### GUI

#### Fixes

- Quick presets resetting to default values
- Sound mode options unsupported by the device visible in quick presets

### Android

#### Fixes

- Rename General menu to Sound Modes

## v1.13.1

### GUI

#### Fixes

- Remove broken gtk binding that was ignored and had no effect other than a critical log message

## v1.13.0

### General

#### Build

- Replace `cargo-make` with `just`

### GUI

#### Features

- Add custom equalizer profile importing/exporting

#### Fixes

- Fix some circumstances where the volume adjustments matching a custom profile would be selected, but the dropdown would not show the profile as selected
- Custom equalizer profile sorting is now case insensitive
- Icon colors now adapt when using dark theme
- Write config file to temp file first before overwriting to work around potential data loss if writing is interrupted
- The config file is no longer overwritten with the exact same content during every application startup

### Android

#### Features

- Add custom equalizer profile importing/exporting
- Show the title of the current screen rather than the device's name in the header bar
- New color scheme generated from logo color

#### Fixes

- To resolve inconsistent transition animations, use slides everywhere
- Custom equalizer profile duplicate detection issues

### Web

#### Features

- Add custom equalizer profile importing/exporting
- There is now a dropdown menu to select light/dark mode in addition to the default option of following the system preference

#### Fixes

- Accesibility issues

## v1.12.0

### CLI

#### Features

- Add shell completions: `openscq30_cli completions`

### Android

#### Features

- Add option to auto connect to paired devices  
  In order to implement this, the companion device API is now used, which means connected devices will no longer be listed by default. They must first be paired with the app. This was necessary to gain permission to start foreground services from the background, which is necessary for auto connect to work.

## v1.11.0

### Android

#### Features

- Add custom button settings
- Reorganize UI to allow for more than 5 total settings pages

### Web

#### Features

- Add custom button settings
- Improve layout on larger screens

## v1.10.6

### General

#### Fixes

- Feature not supported error when changing ambient sound mode cycle even when the feature is supported

### GUI

#### Fixes

- Custom noise canceling option sometimes not visible when it should be, and sometimes visible when it shouldn't be
- Rename button action "Trans" to "Ambient Sound Mode", since that's what it does, cycles through ambient sound modes
- Devices other than Q30 potentially not working on Linux

## v1.10.5

### General

#### Fixes

- Fall back to default values when device is in an invalid state rather than failing to connect

## v1.10.4

### General

#### Fixes

- Fix A3933 and A3939 state update packet parsing

## v1.10.3

### Web

#### Fixes

- Device profile table cell overflowing, breaking layout

## v1.10.2

### Android

#### Fixes

- Various resource leaks when failing to connect to a device
- Edge case where events occur in unexpected order while connecting, causing it to hang

## v1.10.1

### General

#### Fixes

- A3933 equalizer not working

### Android

#### Fixes

- Immediate crash due to misconfigured proguard

## v1.10.0

### General

#### Features

- Support for A3945 (Note 3S), A3933 (Note 3), and A3939 (Life P3)

## v1.9.0

### General

#### Features

- Partial support for A3945, A3933, and A3939. Not ready for general use.

### CLI

#### Fixes

- Panic on exit due to nested tokio runtimes
- "get equalizer" returned values in the range of 0 to 255 rather than -120 to 135
- "set equalizer" accepted values in the range of -12 to 13 rather than -120 to 135

### Web

#### Fixes

- Noise Canceling showing on devices that should not have that option

### Android

#### Fixes

- Custom profiles sometimes not showing as selected despite the equalizer being set correctly

## v1.8.1

### Android

#### Fixes

- Add a few retries and a timeout when connecting to a device

## v1.8.0

### GUI

#### Features

- Add quick presets
- Add button keybinding settings
- Add hear id settings

#### Fixes

- Localize equalizer preset names

#### Dependencies

- Minimum gtk4 version increased to v4.12 (GNOME 45)
- Minimum libadwaita version increased to v1.4 (GNOME 45)

## v1.7.0

### GUI

#### Fixes

- Bluetooth initialization when launching the application will no longer block the window from showing

### Android

#### Features

- If you have more than one type of Soundcore device, they will now each have their own sets of quick presets. To avoid having this be a breaking change, any newly connected devices' quick presets will be initialized with your quick preset configuration prior to this release.

#### Fixes

- Fix settings not used by the connected device showing in quick presets
- Fix screen rotation causing disconnect
- Refresh device list when granting bluetooth permission

## v1.6.0

### General

#### Features

- Add experimental support for new devices: A3027, A3028, A3030, A3033, A3033EU, A3926, A3926Z11, A3930, A3931, A3931XR, A3935, A3935W, A3951
- Add support for custom transparency mode
- Add support for custom noise canceling mode
- Add device information tab

### Android

#### Fixes

- Fix notification buttons not working on API level 34 (Android 14)

### Web

#### Features

- Add loading indicator while in the process of connecting to device

## v1.5.2

### GUI

#### Fixes

- Fix non-soundcore devices showing in device list

### Android

#### Fixes

- Pull to refresh icon now follows color scheme
- No longer crashes on api level 34 (android 14 beta)

### Web

#### Fixes

- Disallow deselecting sound mode buttons

## v1.5.1

### Android

#### Fixes

- Fix noise canceling mode displaying in notification as $1Noise Canceling
- Fix text fields not being single line
- Fix equalizer remaining checked when moving between Quick Preset tabs

## v1.5.0

### GUI

#### Fixes

- Go back to device selection screen immediately when device disconnects

### Android

#### Features

- Added "Quick Presets", which enable changing ambient sound mode, noise canceling mode, and equalizer profile all together with the press of a notification button.

## v1.4.0

### General

#### Fixes

- Improve handing of device disconnects. How this is implemented varies by platform. The desktop GUI could still use some more work.

### Android

- Minimum Android API level increased from 24 to 26 (Android 7.0 to Android 8.0)

#### Features

- The bluetooth connection is now held by a service, so it can be kept open in the background. This is in preparation for having sound mode and eq profile buttons in a notification.

## v1.3.1

### GUI

#### Fixes

- Fix arm64 builds

## v1.3.0

### General

#### Features

- Allow modifying equalizer directly from a preset profile rather than having to switch the dropdown to Custom first
- Allow selecting a custom profile while a preset profile is selected rather than having to select Custom first

### Web

#### Fixes

- Fixed custom profile dropdown having incorrect aria label

## v1.2.0

### General

#### Features

- Add web client (requires Web Bluetooth, currently only supported by Chromium)
- Add equalizer visualization for all profile dropdowns
- Only show create custom profile button when a custom profile is not selected, and only show delete custom profile button when one is selected

#### Fixes

- Fix some Soundcore devices not being detected due to a previously unknown mac address prefix

### GUI

#### Features

- Add volume text input in addition to slider

### Android

#### Features

- Add dialog for replacing existing custom profiles without retyping the name

## v1.1.1

### Android

#### Fixes

- Fixed equalizer number input range being smaller than slider range

## v1.1.0

### General

#### Features

- Equalizer range increased from -6db-+6db to -12db-+12db

### GUI

#### Features

- Add light/dark mode support on Windows

#### Fixes

- Increased default height of window

## v1.0.0

Initial release
