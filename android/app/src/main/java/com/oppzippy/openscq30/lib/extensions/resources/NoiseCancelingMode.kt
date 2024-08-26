package com.oppzippy.openscq30.lib.extensions.resources

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode

fun NoiseCancelingMode.toStringResource(): Int = when (this) {
    NoiseCancelingMode.Transport -> R.string.transport
    NoiseCancelingMode.Outdoor -> R.string.outdoor
    NoiseCancelingMode.Indoor -> R.string.indoor
    NoiseCancelingMode.Custom -> R.string.custom
}
