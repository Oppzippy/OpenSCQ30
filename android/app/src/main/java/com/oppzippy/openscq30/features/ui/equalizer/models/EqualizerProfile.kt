package com.oppzippy.openscq30.features.ui.equalizer.models

import android.util.Log
import com.oppzippy.openscq30.lib.EqualizerBandOffsets
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.PresetEqualizerProfile
import com.oppzippy.openscq30.R

enum class EqualizerProfile(val presetProfile: PresetEqualizerProfile?, val localizationStringId: Int) {
    Custom(null, R.string.custom),
    SoundcoreSignature(PresetEqualizerProfile.SoundcoreSignature, R.string.soundcore_signature),
    Acoustic(PresetEqualizerProfile.Acoustic, R.string.acoustic),
    BassBooster(PresetEqualizerProfile.BassBooster, R.string.bass_booster),
    BassReducer(PresetEqualizerProfile.BassReducer, R.string.bass_reducer),
    Classical(PresetEqualizerProfile.Classical, R.string.classical),
    Podcast(PresetEqualizerProfile.Podcast, R.string.podcast),
    Dance(PresetEqualizerProfile.Dance, R.string.dance),
    Deep(PresetEqualizerProfile.Deep, R.string.deep),
    Electronic(PresetEqualizerProfile.Electronic, R.string.electronic),
    Flat(PresetEqualizerProfile.Flat, R.string.flat),
    HipHop(PresetEqualizerProfile.HipHop, R.string.hip_hop),
    Jazz(PresetEqualizerProfile.Jazz, R.string.jazz),
    Latin(PresetEqualizerProfile.Latin, R.string.latin),
    Lounge(PresetEqualizerProfile.Lounge, R.string.lounge),
    Piano(PresetEqualizerProfile.Piano, R.string.piano),
    Pop(PresetEqualizerProfile.Pop, R.string.pop),
    RnB(PresetEqualizerProfile.RnB, R.string.rnb),
    Rock(PresetEqualizerProfile.Rock, R.string.rock),
    SmallSpeakers(PresetEqualizerProfile.SmallSpeakers, R.string.small_speakers),
    SpokenWord(PresetEqualizerProfile.SpokenWord, R.string.spoken_word),
    TrebleBooster(PresetEqualizerProfile.TrebleBooster, R.string.treble_booster),
    TrebleReducer(PresetEqualizerProfile.TrebleReducer, R.string.treble_reducer);

    fun toEqualizerConfiguration(volumeOffsets: ByteArray?): EqualizerConfiguration {
        return if (presetProfile != null) {
            EqualizerConfiguration(presetProfile)
        } else {
            if (volumeOffsets != null) {
                EqualizerConfiguration(EqualizerBandOffsets(volumeOffsets))
            } else {
                throw NullPointerException("volumeOffsets is null")
            }
        }
    }

    companion object {
        fun fromPresetProfile(presetProfile: PresetEqualizerProfile?): EqualizerProfile {
            EqualizerProfile.values().forEach {
                if (it.presetProfile == presetProfile) {
                    return it
                }
            }
            Log.e(
                "EqualizerProfile",
                "Couldn't find EqualizerProfile for preset $presetProfile, using Custom",
            )
            return Custom
        }
    }
}