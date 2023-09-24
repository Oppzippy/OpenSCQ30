package com.oppzippy.openscq30.features.soundcoredevice.impl

import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.SoundModes

fun filterSoundModeChanges(
    featureFlags: DeviceFeatureFlags,
    prevSoundModes: SoundModes,
    newSoundModes: SoundModes,
): SoundModes {
    return SoundModes(
        if (featureFlags.contains(DeviceFeatureFlags.noiseCancelingMode()) || newSoundModes.ambientSoundMode() != AmbientSoundMode.NoiseCanceling) {
            newSoundModes.ambientSoundMode()
        } else {
            prevSoundModes.ambientSoundMode()
        },
        if (featureFlags.contains(DeviceFeatureFlags.noiseCancelingMode()) &&
            (featureFlags.contains(DeviceFeatureFlags.customNoiseCanceling()) || newSoundModes.noiseCancelingMode() != NoiseCancelingMode.Custom)
        ) {
            newSoundModes.noiseCancelingMode()
        } else {
            prevSoundModes.noiseCancelingMode()
        },
        if (featureFlags.contains(DeviceFeatureFlags.transparencyModes())) {
            newSoundModes.transparencyMode()
        } else {
            prevSoundModes.transparencyMode()
        },
        if (featureFlags.contains(DeviceFeatureFlags.customNoiseCanceling())) {
            newSoundModes.customNoiseCanceling()
        } else {
            prevSoundModes.customNoiseCanceling()
        },
    )
}
