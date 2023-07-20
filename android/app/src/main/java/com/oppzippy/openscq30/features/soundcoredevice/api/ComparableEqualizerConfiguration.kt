package com.oppzippy.openscq30.features.soundcoredevice.api

import com.oppzippy.openscq30.libbindings.EqualizerConfiguration

fun EqualizerConfiguration.contentEquals(
    other: EqualizerConfiguration,
): Boolean {
    val isEqualizerProfileIdEqual = profileId() == other.profileId()

    val thisAdjustments = ArrayList(volumeAdjustments().adjustments().asList())
    val otherAdustments = ArrayList(other.volumeAdjustments().adjustments().asList())
    val areAdjustmentsEqual = thisAdjustments == otherAdustments

    return isEqualizerProfileIdEqual && areAdjustmentsEqual
}
