# UPSTREAM CHANGE REPORT

## Summary

Adds `ACCESS_FINE_LOCATION` permission for Bluetooth paired device discovery on Android 10 (API 29) and Android 11 (API 30), fixing empty device lists on affected OEM ROMs.

## Motivation

On Android 10-11, several OEMs (Samsung, Huawei, etc.) return an empty set from `BluetoothAdapter.getBondedDevices()` when `ACCESS_FINE_LOCATION` is not granted, despite the app using classic RFCOMM (not BLE scanning). This prevents users from seeing their paired headphones in the app on these devices.

This is complementary to the existing `BLUETOOTH_CONNECT` fix (commit `46bdf24`) which addressed the wrong permission being checked on Android <=11.

## Fork URL

https://github.com/shinigami1231111/OpenSCQ30

## Branch

`feature/android-10-11-location-permission`

## Commit Hash

`8f67757a`

## Files Changed

| File | Change Type |
|------|-------------|
| `android/app/src/main/AndroidManifest.xml` | Added |
| `android/app/src/main/java/com/oppzippy/openscq30/ui/deviceselection/DeviceSelectionScreen.kt` | Added + Modified |
| `android/app/src/main/java/com/oppzippy/openscq30/features/soundcoredevice/ConnectionBackends.kt` | Added |
| `android/app/src/main/res/values/strings.xml` | Added |
| `android/app/src/androidTest/java/com/oppzippy/openscq30/TestBase.kt` | Modified |
| `android/app/src/test/java/com/oppzippy/openscq30/ConnectionBackends.kt` | Modified |

## Detailed Technical Explanation

### 1. AndroidManifest.xml

Declares `ACCESS_FINE_LOCATION` with `android:maxSdkVersion="30"`. The permission is only requested on API 29-30 where it is needed. On API 31+, `BLUETOOTH_CONNECT` is used instead.

### 2. DeviceSelectionScreen.kt

Adds a nested `PermissionCheck` for `ACCESS_FINE_LOCATION` when `Build.VERSION.SDK_INT` is in range `Q..R` (29-30). The `NavHost` block is extracted into a `DeviceSelectionContent` private composable to keep the nesting readable. The navigation graph and all callbacks are unchanged.

### 3. ConnectionBackends.kt

Adds runtime `ACCESS_FINE_LOCATION` permission checks in both `devices()` and `connect()` methods, following the same pattern as the existing `BLUETOOTH_CONNECT` check for API 31+. This is a defense-in-depth measure for code paths not reached through the UI permission prompt (e.g., auto-connect service, widget).

### 4. strings.xml

Adds `location_permission_is_required` string resource used in the permission rationale dialog.

### 5. TestBase.kt

Updates `bluetoothPermissionRule` to auto-grant `ACCESS_FINE_LOCATION` on API 29-30 in instrumented tests, using spread operators on `arrayOf(...)`.

### 6. ConnectionBackends.kt (unit test)

Mocks `ACCESS_FINE_LOCATION` permission grant/deny alongside `BLUETOOTH_CONNECT` in the "no permission" test condition.

## Important Implementation Details

- All permission checks are scoped to API 29-30 via `Build.VERSION_CODES.Q..Build.VERSION_CODES.R`.
- `maxSdkVersion="30"` in the manifest prevents the permission from being relevant on Android 12+.
- The `PermissionCheck` composable from `com.oppzippy.openscq30.ui.utils` is reused.
- No Rust/native code is modified.
- No new dependencies are introduced.

## Dependencies

- `com.oppzippy.openscq30.ui.utils.PermissionCheck` (existing composable)
- `com.google.accompanist:accompanist-permissions` (existing dependency)
- `androidx.core.app.ActivityCompat` (existing import)
- `android.Manifest.permission.ACCESS_FINE_LOCATION` (standard Android API)

## Build Verification

- **Build command:** `gradlew assembleDebug --no-daemon`
- **Result:** BUILD SUCCESSFUL in 23s (191 tasks, all cached)
- **ABIs:** arm64-v8a, armeabi-v7a, x86, x86_64
- **Warnings:** Only existing upstream warnings (deprecated API usage, ktlint not found for formatting)

## Test Results

- **Test command:** `gradlew testX86DebugUnitTest --no-daemon`
- **Result:** BUILD SUCCESSFUL in 1m 51s
- **Tests executed:** All unit tests in the project
- **Failures:** None

## Compatibility Considerations

- Android 10 (API 29): Location permission requested, device discovery fixed
- Android 11 (API 30): Location permission requested, device discovery fixed
- Android 12 (API 31): No change, uses existing BLUETOOTH_CONNECT
- Android 13+ (API 33+): No change
- Older than Android 10: Not affected (minSdk is 26, but permission checks only apply to 29-30)

## Known Limitations

- Instrumented tests were not executed on a physical device (require Android emulator or device).
- The `ACCESS_FINE_LOCATION` permission prompt is shown in English only (no translations added for other locales).
- On Android 10-11, users who deny the location permission will see an empty device list (same as current behavior without this fix, but now with a rationale message).

## Pull Request URL

https://github.com/shinigami1231111/OpenSCQ30/pull/new/feature/android-10-11-location-permission
