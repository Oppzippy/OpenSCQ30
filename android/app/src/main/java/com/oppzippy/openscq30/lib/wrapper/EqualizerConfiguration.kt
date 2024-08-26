package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.EqualizerConfiguration as ProtobufEqualizerConfiguration
import com.oppzippy.openscq30.lib.protobuf.PresetEqualizerProfile as ProtobufPresetEqualizerProfile
import com.oppzippy.openscq30.lib.protobuf.StereoVolumeAdjustments as ProtobufStereoVolumeAdjustments
import com.oppzippy.openscq30.lib.protobuf.equalizerConfiguration
import com.oppzippy.openscq30.lib.protobuf.stereoVolumeAdjustments

data class EqualizerConfiguration(
    val presetProfile: PresetEqualizerProfile? = null,
    val volumeAdjustments: List<Double>,
) {
    fun toProtobuf(): ProtobufEqualizerConfiguration = equalizerConfiguration {
        this@EqualizerConfiguration.presetProfile?.let { presetProfile = it.toProtobuf() }
        volumeAdjustments.addAll(this@EqualizerConfiguration.volumeAdjustments)
    }
}

fun ProtobufEqualizerConfiguration.toKotlin(): EqualizerConfiguration = EqualizerConfiguration(
    presetProfile = if (hasPresetProfile()) presetProfile.toKotlin() else null,
    volumeAdjustments = volumeAdjustmentsList,
)

data class StereoVolumeAdjustments(val left: List<Double>, val right: List<Double>) {
    fun toProtobuf(): ProtobufStereoVolumeAdjustments = stereoVolumeAdjustments {
        left.addAll(this@StereoVolumeAdjustments.left)
        right.addAll(this@StereoVolumeAdjustments.right)
    }
}

fun ProtobufStereoVolumeAdjustments.toKotlin(): StereoVolumeAdjustments = StereoVolumeAdjustments(
    left = leftList,
    right = rightList,
)

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

    fun toProtobuf(): ProtobufPresetEqualizerProfile = when (this) {
        SoundcoreSignature -> ProtobufPresetEqualizerProfile.SOUNDCORE_SIGNATURE
        Acoustic -> ProtobufPresetEqualizerProfile.ACOUSTIC
        BassBooster -> ProtobufPresetEqualizerProfile.BASS_BOOSTER
        BassReducer -> ProtobufPresetEqualizerProfile.BASS_REDUCER
        Classical -> ProtobufPresetEqualizerProfile.CLASSICAL
        Podcast -> ProtobufPresetEqualizerProfile.PODCAST
        Dance -> ProtobufPresetEqualizerProfile.DANCE
        Deep -> ProtobufPresetEqualizerProfile.DEEP
        Electronic -> ProtobufPresetEqualizerProfile.ELECTRONIC
        Flat -> ProtobufPresetEqualizerProfile.FLAT
        HipHop -> ProtobufPresetEqualizerProfile.HIP_HOP
        Jazz -> ProtobufPresetEqualizerProfile.JAZZ
        Latin -> ProtobufPresetEqualizerProfile.LATIN
        Lounge -> ProtobufPresetEqualizerProfile.LOUNGE
        Piano -> ProtobufPresetEqualizerProfile.PIANO
        Pop -> ProtobufPresetEqualizerProfile.POP
        RnB -> ProtobufPresetEqualizerProfile.RNB
        Rock -> ProtobufPresetEqualizerProfile.ROCK
        SmallSpeakers -> ProtobufPresetEqualizerProfile.SMALL_SPEAKERS
        SpokenWord -> ProtobufPresetEqualizerProfile.SPOKEN_WORD
        TrebleBooster -> ProtobufPresetEqualizerProfile.TREBLE_BOOSTER
        TrebleReducer -> ProtobufPresetEqualizerProfile.TREBLE_REDUCER
    }
}

fun ProtobufPresetEqualizerProfile.toKotlin(): PresetEqualizerProfile = when (this) {
    ProtobufPresetEqualizerProfile.SOUNDCORE_SIGNATURE -> PresetEqualizerProfile.SoundcoreSignature
    ProtobufPresetEqualizerProfile.ACOUSTIC -> PresetEqualizerProfile.Acoustic
    ProtobufPresetEqualizerProfile.BASS_BOOSTER -> PresetEqualizerProfile.BassBooster
    ProtobufPresetEqualizerProfile.BASS_REDUCER -> PresetEqualizerProfile.BassReducer
    ProtobufPresetEqualizerProfile.CLASSICAL -> PresetEqualizerProfile.Classical
    ProtobufPresetEqualizerProfile.PODCAST -> PresetEqualizerProfile.Podcast
    ProtobufPresetEqualizerProfile.DANCE -> PresetEqualizerProfile.Dance
    ProtobufPresetEqualizerProfile.DEEP -> PresetEqualizerProfile.Deep
    ProtobufPresetEqualizerProfile.ELECTRONIC -> PresetEqualizerProfile.Electronic
    ProtobufPresetEqualizerProfile.FLAT -> PresetEqualizerProfile.Flat
    ProtobufPresetEqualizerProfile.HIP_HOP -> PresetEqualizerProfile.HipHop
    ProtobufPresetEqualizerProfile.JAZZ -> PresetEqualizerProfile.Jazz
    ProtobufPresetEqualizerProfile.LATIN -> PresetEqualizerProfile.Latin
    ProtobufPresetEqualizerProfile.LOUNGE -> PresetEqualizerProfile.Lounge
    ProtobufPresetEqualizerProfile.PIANO -> PresetEqualizerProfile.Piano
    ProtobufPresetEqualizerProfile.POP -> PresetEqualizerProfile.Pop
    ProtobufPresetEqualizerProfile.RNB -> PresetEqualizerProfile.RnB
    ProtobufPresetEqualizerProfile.ROCK -> PresetEqualizerProfile.Rock
    ProtobufPresetEqualizerProfile.SMALL_SPEAKERS -> PresetEqualizerProfile.SmallSpeakers
    ProtobufPresetEqualizerProfile.SPOKEN_WORD -> PresetEqualizerProfile.SpokenWord
    ProtobufPresetEqualizerProfile.TREBLE_BOOSTER -> PresetEqualizerProfile.TrebleBooster
    ProtobufPresetEqualizerProfile.TREBLE_REDUCER -> PresetEqualizerProfile.TrebleReducer
}
