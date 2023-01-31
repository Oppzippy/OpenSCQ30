package com.oppzippy.openscq30.features.ui.equalizer

import kotlin.jvm.optionals.getOrNull

data class EqualizerConfiguration(
    val equalizerProfile: EqualizerProfile,
    val values: List<Byte>,
) {
    fun toRust(): com.oppzippy.openscq30.lib.EqualizerConfiguration {
        return equalizerProfile.toEqualizerConfiguration(values.toByteArray())
    }

    companion object {
        fun fromRust(configuration: com.oppzippy.openscq30.lib.EqualizerConfiguration): EqualizerConfiguration {
            return EqualizerConfiguration(
                EqualizerProfile.fromPresetProfile(configuration.presetProfile().getOrNull()),
                configuration.bandOffsets().volumeOffsets().asList(),
            )
        }
    }
}