package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.AdaptiveNoiseCanceling as ProtobufAdaptiveNoiseCanceling
import com.oppzippy.openscq30.lib.protobuf.ManualNoiseCanceling as ProtobufManualNoiseCanceling
import com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeTypeTwo as ProtobufNoiseCancelingModeTypeTwo
import com.oppzippy.openscq30.lib.protobuf.SoundModesTypeTwo as ProtobufSoundModesTypeTwo
import com.oppzippy.openscq30.lib.protobuf.soundModesTypeTwo

data class SoundModesTypeTwo(
    val ambientSoundMode: AmbientSoundMode,
    val transparencyMode: TransparencyMode,
    val adaptiveNoiseCanceling: AdaptiveNoiseCanceling,
    val manualNoiseCanceling: ManualNoiseCanceling,
    val noiseCancelingMode: NoiseCancelingModeTypeTwo,
    val windNoiseSuppression: Boolean,
    val detectedWindNoise: Boolean,
    val noiseCancelingAdaptiveSensitivityLevel: UByte,
) {
    fun toProtobuf(): ProtobufSoundModesTypeTwo = soundModesTypeTwo {
        ambientSoundMode = this@SoundModesTypeTwo.ambientSoundMode.toProtobuf()
        transparencyMode = this@SoundModesTypeTwo.transparencyMode.toProtobuf()
        adaptiveNoiseCanceling = this@SoundModesTypeTwo.adaptiveNoiseCanceling.toProtobuf()
        manualNoiseCanceling = this@SoundModesTypeTwo.manualNoiseCanceling.toProtobuf()
        noiseCancelingMode = this@SoundModesTypeTwo.noiseCancelingMode.toProtobuf()
        windNoiseSuppression = this@SoundModesTypeTwo.windNoiseSuppression
        detectedWindNoise = this@SoundModesTypeTwo.detectedWindNoise
        noiseCancelingAdaptiveSensitivityLevel =
            this@SoundModesTypeTwo.noiseCancelingAdaptiveSensitivityLevel.toInt()
    }
}

fun ProtobufSoundModesTypeTwo.toKotlin(): SoundModesTypeTwo = SoundModesTypeTwo(
    ambientSoundMode = ambientSoundMode.toKotlin(),
    transparencyMode = transparencyMode.toKotlin(),
    adaptiveNoiseCanceling = adaptiveNoiseCanceling.toKotlin(),
    manualNoiseCanceling = manualNoiseCanceling.toKotlin(),
    noiseCancelingMode = noiseCancelingMode.toKotlin(),
    windNoiseSuppression = windNoiseSuppression,
    detectedWindNoise = detectedWindNoise,
    noiseCancelingAdaptiveSensitivityLevel = noiseCancelingAdaptiveSensitivityLevel.toUByte(),
)

enum class AdaptiveNoiseCanceling {
    LowNoise,
    MediumNoise,
    HighNoise,
    ;

    fun toProtobuf(): ProtobufAdaptiveNoiseCanceling = when (this) {
        LowNoise -> ProtobufAdaptiveNoiseCanceling.LOW_NOISE
        MediumNoise -> ProtobufAdaptiveNoiseCanceling.MEDIUM_NOISE
        HighNoise -> ProtobufAdaptiveNoiseCanceling.HIGH_NOISE
    }
}

fun ProtobufAdaptiveNoiseCanceling.toKotlin(): AdaptiveNoiseCanceling = when (this) {
    ProtobufAdaptiveNoiseCanceling.LOW_NOISE -> AdaptiveNoiseCanceling.LowNoise
    ProtobufAdaptiveNoiseCanceling.MEDIUM_NOISE -> AdaptiveNoiseCanceling.MediumNoise
    ProtobufAdaptiveNoiseCanceling.HIGH_NOISE -> AdaptiveNoiseCanceling.HighNoise
}

enum class ManualNoiseCanceling {
    Weak,
    Moderate,
    Strong,
    ;

    fun toProtobuf(): ProtobufManualNoiseCanceling = when (this) {
        Weak -> ProtobufManualNoiseCanceling.WEAK
        Moderate -> ProtobufManualNoiseCanceling.MODERATE
        Strong -> ProtobufManualNoiseCanceling.STRONG
    }
}

fun ProtobufManualNoiseCanceling.toKotlin(): ManualNoiseCanceling = when (this) {
    ProtobufManualNoiseCanceling.WEAK -> ManualNoiseCanceling.Weak
    ProtobufManualNoiseCanceling.MODERATE -> ManualNoiseCanceling.Moderate
    ProtobufManualNoiseCanceling.STRONG -> ManualNoiseCanceling.Strong
}

enum class NoiseCancelingModeTypeTwo {
    Adaptive,
    Manual,
    ;

    fun toProtobuf(): ProtobufNoiseCancelingModeTypeTwo = when (this) {
        Adaptive -> ProtobufNoiseCancelingModeTypeTwo.ADAPTIVE
        Manual -> ProtobufNoiseCancelingModeTypeTwo.MANUAL
    }
}

fun ProtobufNoiseCancelingModeTypeTwo.toKotlin(): NoiseCancelingModeTypeTwo = when (this) {
    ProtobufNoiseCancelingModeTypeTwo.ADAPTIVE -> NoiseCancelingModeTypeTwo.Adaptive
    ProtobufNoiseCancelingModeTypeTwo.MANUAL -> NoiseCancelingModeTypeTwo.Manual
}
