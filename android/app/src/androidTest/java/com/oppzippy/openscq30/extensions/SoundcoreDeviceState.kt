package com.oppzippy.openscq30.extensions

import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState

fun SoundcoreDeviceState.Companion.empty(): SoundcoreDeviceState {
    return SoundcoreDeviceState(
        featureFlags = DeviceFeatureFlags.empty(),
        serialNumber = null,
        leftFirmwareVersion = null,
        rightFirmwareVersion = null,
        equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
        soundModes = null,
        leftBatteryLevel = 0,
        rightBatteryLevel = 0,
        isLeftBatteryCharging = false,
        isRightBatteryCharging = false,
        ageRange = null,
        dynamicRangeCompressionMinFirmwareVersion = null,
        gender = null,
        customHearId = null,
    )
}
