# Changelog

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
