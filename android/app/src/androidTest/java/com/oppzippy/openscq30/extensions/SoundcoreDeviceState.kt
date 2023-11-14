package com.oppzippy.openscq30.extensions

import com.oppzippy.openscq30.lib.bindings.DeviceProfile
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState

fun SoundcoreDeviceState.Companion.empty(): SoundcoreDeviceState {
    return SoundcoreDeviceState(
        deviceProfile = DeviceProfile(
            null,
            false,
            0,
            0,
            false,
            false,
            false,
            false,
            false,
            null,
        ),
        serialNumber = null,
        firmwareVersion = null,
        equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
        soundModes = null,
        leftBatteryLevel = 0,
        rightBatteryLevel = 0,
        isLeftBatteryCharging = false,
        isRightBatteryCharging = false,
        ageRange = null,
        gender = null,
        customHearId = null,
    )
}
