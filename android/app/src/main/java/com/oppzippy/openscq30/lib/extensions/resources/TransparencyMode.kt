package com.oppzippy.openscq30.lib.extensions.resources

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode

fun TransparencyMode.toStringResource(): Int = when (this) {
    TransparencyMode.VocalMode -> R.string.vocal_mode
    TransparencyMode.FullyTransparent -> R.string.fully_transparent
}
