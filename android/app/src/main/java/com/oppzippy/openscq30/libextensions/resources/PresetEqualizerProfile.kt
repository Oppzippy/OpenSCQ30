package com.oppzippy.openscq30.libextensions.resources

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.libbindings.PresetEqualizerProfile

fun PresetEqualizerProfile.toStringResource(): Int {
    return when (this) {
        PresetEqualizerProfile.SoundcoreSignature -> R.string.soundcore_signature
        PresetEqualizerProfile.Acoustic -> R.string.acoustic
        PresetEqualizerProfile.BassBooster -> R.string.bass_booster
        PresetEqualizerProfile.BassReducer -> R.string.bass_reducer
        PresetEqualizerProfile.Classical -> R.string.classical
        PresetEqualizerProfile.Podcast -> R.string.podcast
        PresetEqualizerProfile.Dance -> R.string.dance
        PresetEqualizerProfile.Deep -> R.string.deep
        PresetEqualizerProfile.Electronic -> R.string.electronic
        PresetEqualizerProfile.Flat -> R.string.flat
        PresetEqualizerProfile.HipHop -> R.string.hip_hop
        PresetEqualizerProfile.Jazz -> R.string.jazz
        PresetEqualizerProfile.Latin -> R.string.latin
        PresetEqualizerProfile.Lounge -> R.string.lounge
        PresetEqualizerProfile.Piano -> R.string.piano
        PresetEqualizerProfile.Pop -> R.string.pop
        PresetEqualizerProfile.RnB -> R.string.rnb
        PresetEqualizerProfile.Rock -> R.string.rock
        PresetEqualizerProfile.SmallSpeakers -> R.string.small_speakers
        PresetEqualizerProfile.SpokenWord -> R.string.spoken_word
        PresetEqualizerProfile.TrebleBooster -> R.string.treble_booster
        PresetEqualizerProfile.TrebleReducer -> R.string.treble_reducer
    }
}
