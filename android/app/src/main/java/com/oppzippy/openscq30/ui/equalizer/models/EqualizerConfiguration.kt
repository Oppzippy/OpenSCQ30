package com.oppzippy.openscq30.ui.equalizer.models

import kotlin.jvm.optionals.getOrNull

data class EqualizerConfiguration(
    val equalizerProfile: EqualizerProfile,
    val values: List<Byte>,
) {
    fun toRust(): com.oppzippy.openscq30.libbindings.EqualizerConfiguration {
        return equalizerProfile.toEqualizerConfiguration(values.toByteArray())
    }

    companion object {
        fun fromRust(configuration: com.oppzippy.openscq30.libbindings.EqualizerConfiguration): EqualizerConfiguration {
            return EqualizerConfiguration(
                EqualizerProfile.fromPresetProfile(configuration.presetProfile().getOrNull()),
                configuration.volumeAdjustments().adjustments().asList(),
            )
        }
    }
}
