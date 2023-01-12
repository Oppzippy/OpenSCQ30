package com.oppzippy.openscq30.ui.equalizer

class ProfileIndexTracker(profiles: Array<EqualizerProfile>) {
    private val volumeOffsetsToIndex = HashMap<ULong, Int>()
    private var customProfileIndex = -1

    init {
        profiles.forEachIndexed { index, equalizerProfile ->
            if (equalizerProfile != EqualizerProfile.Custom) {
                val volumeOffsets =
                    equalizerProfile.toEqualizerConfiguration(null).bandOffsets().volumeOffsets()
                val id = arrayToULong(volumeOffsets)
                volumeOffsetsToIndex[id] = index
            } else {
                customProfileIndex = index
            }
        }
    }

    operator fun get(volumeOffsets: ByteArray): Int {
        val id = arrayToULong(volumeOffsets)
        return volumeOffsetsToIndex[id] ?: customProfileIndex
    }

    private fun arrayToULong(array: ByteArray): ULong {
        val ulong = array.foldIndexed(0UL) { index, acc, byte ->
            val ulong = byte.toUByte().toULong()
            acc.or(ulong.shl(index * 8))
        }
        return ulong
    }
}