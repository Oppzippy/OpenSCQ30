package com.oppzippy.openscq30.lib.extensions.resources

import com.oppzippy.openscq30.lib.bindings.newEqualizerConfigurationFromPresetProfile
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.PresetEqualizerProfile

fun PresetEqualizerProfile.toEqualizerConfiguration(): EqualizerConfiguration =
    newEqualizerConfigurationFromPresetProfile(this)
