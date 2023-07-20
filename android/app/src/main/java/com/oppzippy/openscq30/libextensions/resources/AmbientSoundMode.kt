package com.oppzippy.openscq30.libextensions.resources

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.libbindings.AmbientSoundMode

fun AmbientSoundMode.toStringResource(): Int {
    return when (this) {
        AmbientSoundMode.NoiseCanceling -> R.string.noise_canceling
        AmbientSoundMode.Transparency -> R.string.transparency
        AmbientSoundMode.Normal -> R.string.normal
    }
}
