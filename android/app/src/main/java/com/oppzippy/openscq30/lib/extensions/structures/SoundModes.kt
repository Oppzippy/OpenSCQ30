package com.oppzippy.openscq30.lib.extensions.structures

import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.TransparencyMode

fun SoundModes.copy(
    ambientSoundMode: AmbientSoundMode = this.ambientSoundMode(),
    noiseCancelingMode: NoiseCancelingMode = this.noiseCancelingMode(),
    transparencyMode: TransparencyMode = this.transparencyMode(),
    customNoiseCanceling: CustomNoiseCanceling = this.customNoiseCanceling(),
): SoundModes {
    return SoundModes(
        ambientSoundMode,
        noiseCancelingMode,
        transparencyMode,
        customNoiseCanceling,
    )
}
