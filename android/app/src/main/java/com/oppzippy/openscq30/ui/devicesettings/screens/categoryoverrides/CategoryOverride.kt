package com.oppzippy.openscq30.ui.devicesettings.screens.categoryoverrides

import androidx.compose.runtime.Composable
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value

val categoryScreenOverrides: Map<String, List<CategoryOverride>> = mapOf(
    "equalizer" to listOf(SoundcoreEqualizerScreen),
)

interface CategoryOverride {
    fun shouldOverride(deviceModel: String, settings: List<Pair<String, Setting>>): Boolean

    @Composable
    fun Screen(settings: List<Pair<String, Setting>>, setSettings: (List<Pair<String, Value>>) -> Unit)
}
