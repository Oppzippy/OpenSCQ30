package com.oppzippy.openscq30.features.soundcoredevice.api

import com.oppzippy.openscq30.lib.EqualizerConfiguration

fun EqualizerConfiguration.contentEquals(
    other: EqualizerConfiguration
): Boolean {
    val isEqualizerProfileIdEqual = profileId() == other.profileId()

    val thisBandOffsets = ArrayList(bandOffsets().volumeOffsets().asList())
    val otherBandOffsets = ArrayList(other.bandOffsets().volumeOffsets().asList())
    val areEqualizerBandOffsetsEqual = thisBandOffsets == otherBandOffsets

    return isEqualizerProfileIdEqual && areEqualizerBandOffsetsEqual
}