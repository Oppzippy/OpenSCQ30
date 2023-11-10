package com.oppzippy.openscq30.features.soundcoredevice.impl

import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingModeType
import com.oppzippy.openscq30.lib.bindings.SoundModeProfile
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.TransparencyModeType

fun filterSoundModeChanges(
    soundModeProfile: SoundModeProfile,
    prevSoundModes: SoundModes,
    newSoundModes: SoundModes,
): SoundModes {
    return SoundModes(
        if (soundModeProfile.noiseCancelingModeType() != NoiseCancelingModeType.None || newSoundModes.ambientSoundMode() != AmbientSoundMode.NoiseCanceling) {
            newSoundModes.ambientSoundMode()
        } else {
            prevSoundModes.ambientSoundMode()
        },
        if (soundModeProfile.noiseCancelingModeType() != NoiseCancelingModeType.None &&
            (soundModeProfile.noiseCancelingModeType() == NoiseCancelingModeType.Custom || newSoundModes.noiseCancelingMode() != NoiseCancelingMode.Custom)
        ) {
            newSoundModes.noiseCancelingMode()
        } else {
            prevSoundModes.noiseCancelingMode()
        },
        if (soundModeProfile.transparencyModeType() == TransparencyModeType.Custom) {
            newSoundModes.transparencyMode()
        } else {
            prevSoundModes.transparencyMode()
        },
        if (soundModeProfile.noiseCancelingModeType() == NoiseCancelingModeType.Custom) {
            newSoundModes.customNoiseCanceling()
        } else {
            prevSoundModes.customNoiseCanceling()
        },
    )
}
