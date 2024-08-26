package com.oppzippy.openscq30.ui.equalizer.models

import android.util.Log
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.extensions.resources.toEqualizerConfiguration
import com.oppzippy.openscq30.lib.extensions.resources.toStringResource
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.PresetEqualizerProfile

enum class EqualizerProfile(val presetProfile: PresetEqualizerProfile?) {
    Custom(null),
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
    ;

    val localizationStringId: Int
        get() {
            return presetProfile?.toStringResource() ?: R.string.custom
        }

    fun toEqualizerConfiguration(volumeAdjustments: List<Double>?): EqualizerConfiguration =
        presetProfile?.toEqualizerConfiguration()
            ?: if (volumeAdjustments != null) {
                EqualizerConfiguration(volumeAdjustments = volumeAdjustments)
            } else {
                throw NullPointerException("volumeAdjustments is null")
            }
}

fun PresetEqualizerProfile?.toEqualizerProfile(): EqualizerProfile {
    EqualizerProfile.entries.forEach {
        if (it.presetProfile == this) {
            return it
        }
    }
    Log.e(
        "EqualizerProfile",
        "Couldn't find EqualizerProfile for preset $this, using Custom",
    )
    return EqualizerProfile.Custom
}
