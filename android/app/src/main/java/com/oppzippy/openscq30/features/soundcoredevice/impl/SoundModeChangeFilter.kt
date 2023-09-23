package com.oppzippy.openscq30.features.soundcoredevice.impl

import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.SoundModes

fun filterSoundModeChanges(
    featureFlags: Int,
    prevSoundModes: SoundModes,
    newSoundModes: SoundModes,
): SoundModes {
    return SoundModes(
        if (featureFlags and DeviceFeatureFlags.noiseCancelingMode() == 0 && newSoundModes.ambientSoundMode() == AmbientSoundMode.NoiseCanceling) {
            prevSoundModes.ambientSoundMode()
        } else {
            newSoundModes.ambientSoundMode()
        },
        if (featureFlags and DeviceFeatureFlags.noiseCancelingMode() == 0 ||
            (featureFlags and DeviceFeatureFlags.customNoiseCanceling() == 0 && newSoundModes.noiseCancelingMode() == NoiseCancelingMode.Custom)
        ) {
            prevSoundModes.noiseCancelingMode()
        } else {
            newSoundModes.noiseCancelingMode()
        },
        if (featureFlags and DeviceFeatureFlags.transparencyModes() == 0) {
            prevSoundModes.transparencyMode()
        } else {
            newSoundModes.transparencyMode()
        },
        if (featureFlags and DeviceFeatureFlags.customNoiseCanceling() == 0) {
            prevSoundModes.customNoiseCanceling()
        } else {
            newSoundModes.customNoiseCanceling()
        },
    )
}
