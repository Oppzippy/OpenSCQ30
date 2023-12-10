package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.equalizerConfiguration
import com.oppzippy.openscq30.lib.protobuf.stereoVolumeAdjustments

data class EqualizerConfiguration(
    val presetProfile: PresetEqualizerProfile? = null,
    val volumeAdjustments: List<Double>,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.EqualizerConfiguration {
        return equalizerConfiguration {
            this@EqualizerConfiguration.presetProfile?.let { presetProfile = it.toProtobuf() }
            volumeAdjustments.addAll(this@EqualizerConfiguration.volumeAdjustments)
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.EqualizerConfiguration.toKotlin(): EqualizerConfiguration {
    return EqualizerConfiguration(
        presetProfile = if (hasPresetProfile()) presetProfile.toKotlin() else null,
        volumeAdjustments = volumeAdjustmentsList,
    )
}

data class StereoVolumeAdjustments(val left: List<Double>, val right: List<Double>) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.StereoVolumeAdjustments {
        return stereoVolumeAdjustments {
            left.addAll(this@StereoVolumeAdjustments.left)
            right.addAll(this@StereoVolumeAdjustments.right)
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.StereoVolumeAdjustments.toKotlin(): StereoVolumeAdjustments {
    return StereoVolumeAdjustments(
        left = leftList,
        right = rightList,
    )
}

enum class PresetEqualizerProfile {
    SoundcoreSignature,
    Acoustic,
    BassBooster,
    BassReducer,
    Classical,
    Podcast,
    Dance,
    Deep,
    Electronic,
    Flat,
    HipHop,
    Jazz,
    Latin,
    Lounge,
    Piano,
    Pop,
    RnB,
    Rock,
    SmallSpeakers,
    SpokenWord,
    TrebleBooster,
    TrebleReducer,
    ;

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile {
        return when (this) {
            SoundcoreSignature -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.SOUNDCORE_SIGNATURE
            Acoustic -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.ACOUSTIC
            BassBooster -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.BASS_BOOSTER
            BassReducer -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.BASS_REDUCER
            Classical -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.CLASSICAL
            Podcast -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.PODCAST
            Dance -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.DANCE
            Deep -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.DEEP
            Electronic -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.ELECTRONIC
            Flat -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.FLAT
            HipHop -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.HIP_HOP
            Jazz -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.JAZZ
            Latin -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.LATIN
            Lounge -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.LOUNGE
            Piano -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.PIANO
            Pop -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.POP
            RnB -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.RNB
            Rock -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.ROCK
            SmallSpeakers -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.SMALL_SPEAKERS
            SpokenWord -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.SPOKEN_WORD
            TrebleBooster -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.TREBLE_BOOSTER
            TrebleReducer -> com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.TREBLE_REDUCER
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.toKotlin(): PresetEqualizerProfile {
    return when (this) {
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.SOUNDCORE_SIGNATURE -> PresetEqualizerProfile.SoundcoreSignature
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.ACOUSTIC -> PresetEqualizerProfile.Acoustic
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.BASS_BOOSTER -> PresetEqualizerProfile.BassBooster
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.BASS_REDUCER -> PresetEqualizerProfile.BassReducer
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.CLASSICAL -> PresetEqualizerProfile.Classical
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.PODCAST -> PresetEqualizerProfile.Podcast
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.DANCE -> PresetEqualizerProfile.Dance
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.DEEP -> PresetEqualizerProfile.Deep
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.ELECTRONIC -> PresetEqualizerProfile.Electronic
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.FLAT -> PresetEqualizerProfile.Flat
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.HIP_HOP -> PresetEqualizerProfile.HipHop
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.JAZZ -> PresetEqualizerProfile.Jazz
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.LATIN -> PresetEqualizerProfile.Latin
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.LOUNGE -> PresetEqualizerProfile.Lounge
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.PIANO -> PresetEqualizerProfile.Piano
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.POP -> PresetEqualizerProfile.Pop
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.RNB -> PresetEqualizerProfile.RnB
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.ROCK -> PresetEqualizerProfile.Rock
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.SMALL_SPEAKERS -> PresetEqualizerProfile.SmallSpeakers
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.SPOKEN_WORD -> PresetEqualizerProfile.SpokenWord
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.TREBLE_BOOSTER -> PresetEqualizerProfile.TrebleBooster
        com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile.TREBLE_REDUCER -> PresetEqualizerProfile.TrebleReducer
    }
}
