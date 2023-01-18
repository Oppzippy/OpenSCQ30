package com.oppzippy.openscq30.ui.devicesettings

import android.util.Log
import com.oppzippy.openscq30.lib.EqualizerBandOffsets
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.PresetEqualizerProfile

enum class EqualizerProfile(val presetProfile: PresetEqualizerProfile?) {
    SoundcoreSignature(PresetEqualizerProfile.SoundcoreSignature),
    Acoustic(PresetEqualizerProfile.Acoustic),
    BassBooster(PresetEqualizerProfile.BassBooster),
    BassReducer(PresetEqualizerProfile.BassReducer),
    Classical(PresetEqualizerProfile.Classical),
    Podcast(PresetEqualizerProfile.Podcast),
    Dance(PresetEqualizerProfile.Dance),
    Deep(PresetEqualizerProfile.Deep),
    Electronic(PresetEqualizerProfile.Electronic),
    Flat(PresetEqualizerProfile.Flat),
    HipHop(PresetEqualizerProfile.HipHop),
    Jazz(PresetEqualizerProfile.Jazz),
    Latin(PresetEqualizerProfile.Latin),
    Lounge(PresetEqualizerProfile.Lounge),
    Piano(PresetEqualizerProfile.Piano),
    Pop(PresetEqualizerProfile.Pop),
    RnB(PresetEqualizerProfile.RnB),
    Rock(PresetEqualizerProfile.Rock),
    SmallSpeakers(PresetEqualizerProfile.SmallSpeakers),
    SpokenWord(PresetEqualizerProfile.SpokenWord),
    TrebleBooster(PresetEqualizerProfile.TrebleBooster),
    TrebleReducer(PresetEqualizerProfile.TrebleReducer),
    Custom(null);

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