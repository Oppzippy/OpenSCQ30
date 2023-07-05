package com.oppzippy.openscq30.libextensions.resources

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.NoiseCancelingMode

fun NoiseCancelingMode.toStringResource(): Int {
    return when (this) {
        NoiseCancelingMode.Transport -> R.string.transport
        NoiseCancelingMode.Outdoor -> R.string.outdoor
        NoiseCancelingMode.Indoor -> R.string.indoor
    }
}
