package com.oppzippy.openscq30.extensions

import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState

fun SoundcoreDeviceState.Companion.empty(): SoundcoreDeviceState {
    return SoundcoreDeviceState(
        featureFlags = 0,
        serialNumber = null,
        firmwareVersion = null,
        equalizerConfiguration = EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
        soundModes = null,
    )
}
