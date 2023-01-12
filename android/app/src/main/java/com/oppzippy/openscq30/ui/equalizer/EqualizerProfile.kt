package com.oppzippy.openscq30.ui.equalizer

import com.oppzippy.openscq30.lib.EqualizerBandOffsets
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.PresetEqualizerProfile

enum class EqualizerProfile {
    SoundcoreSignature, Acoustic, BassBooster, BassReducer, Classical, Podcast, Dance, Deep, Electronic, Flat, HipHop, Jazz, Latin, Lounge, Piano, Pop, RnB, Rock, SmallSpeakers, SpokenWord, TrebleBooster, TrebleReducer, Custom;

    fun toEqualizerConfiguration(volumeOffsets: ByteArray?): EqualizerConfiguration {
        return when (this) {
            SoundcoreSignature -> EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature)
            Acoustic -> EqualizerConfiguration(PresetEqualizerProfile.Acoustic)
            BassBooster -> EqualizerConfiguration(PresetEqualizerProfile.BassBooster)
            BassReducer -> EqualizerConfiguration(PresetEqualizerProfile.BassReducer)
            Classical -> EqualizerConfiguration(PresetEqualizerProfile.Classical)
            Podcast -> EqualizerConfiguration(PresetEqualizerProfile.Podcast)
            Dance -> EqualizerConfiguration(PresetEqualizerProfile.Dance)
            Deep -> EqualizerConfiguration(PresetEqualizerProfile.Deep)
            Electronic -> EqualizerConfiguration(PresetEqualizerProfile.Electronic)
            Flat -> EqualizerConfiguration(PresetEqualizerProfile.Flat)
            HipHop -> EqualizerConfiguration(PresetEqualizerProfile.HipHop)
            Jazz -> EqualizerConfiguration(PresetEqualizerProfile.Jazz)
            Latin -> EqualizerConfiguration(PresetEqualizerProfile.Latin)
            Lounge -> EqualizerConfiguration(PresetEqualizerProfile.Lounge)
            Piano -> EqualizerConfiguration(PresetEqualizerProfile.Piano)
            Pop -> EqualizerConfiguration(PresetEqualizerProfile.Pop)
            RnB -> EqualizerConfiguration(PresetEqualizerProfile.RnB)
            Rock -> EqualizerConfiguration(PresetEqualizerProfile.Rock)
            SmallSpeakers -> EqualizerConfiguration(PresetEqualizerProfile.SmallSpeakers)
            SpokenWord -> EqualizerConfiguration(PresetEqualizerProfile.SpokenWord)
            TrebleBooster -> EqualizerConfiguration(PresetEqualizerProfile.TrebleBooster)
            TrebleReducer -> EqualizerConfiguration(PresetEqualizerProfile.TrebleReducer)
            Custom -> if (volumeOffsets != null) {
                EqualizerConfiguration(EqualizerBandOffsets(volumeOffsets))
            } else {
                throw NullPointerException("volumeOffsets is null")
            }
        }
    }
}