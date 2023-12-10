package com.oppzippy.openscq30.ui.quickpresets.models

import com.oppzippy.openscq30.lib.wrapper.PresetEqualizerProfile

sealed class QuickPresetEqualizerConfiguration {
    class PresetProfile(val profile: PresetEqualizerProfile) : QuickPresetEqualizerConfiguration()
    class CustomProfile(val name: String) : QuickPresetEqualizerConfiguration()
}
