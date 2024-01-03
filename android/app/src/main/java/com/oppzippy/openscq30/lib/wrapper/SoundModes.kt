package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.ambientSoundModeCycle
import com.oppzippy.openscq30.lib.protobuf.soundModes

data class SoundModes(
    val ambientSoundMode: AmbientSoundMode,
    val noiseCancelingMode: NoiseCancelingMode,
    val transparencyMode: TransparencyMode,
    val customNoiseCanceling: UByte,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.SoundModes {
        return soundModes {
            ambientSoundMode = this@SoundModes.ambientSoundMode.toProtobuf()
            noiseCancelingMode = this@SoundModes.noiseCancelingMode.toProtobuf()
            transparencyMode = this@SoundModes.transparencyMode.toProtobuf()
            customNoiseCanceling = this@SoundModes.customNoiseCanceling.toInt()
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.SoundModes.toKotlin(): SoundModes {
    return SoundModes(
        ambientSoundMode = ambientSoundMode.toKotlin(),
        noiseCancelingMode = noiseCancelingMode.toKotlin(),
        transparencyMode = transparencyMode.toKotlin(),
        customNoiseCanceling = customNoiseCanceling.toUByte(),
    )
}

enum class AmbientSoundMode {
    Normal,
    Transparency,
    NoiseCanceling,
    ;

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode {
        return when (this) {
            Normal -> com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode.NORMAL
            Transparency -> com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode.TRANSPARENCY
            NoiseCanceling -> com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode.NOISE_CANCELING
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode.toKotlin(): AmbientSoundMode {
    return when (this) {
        com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode.NORMAL -> AmbientSoundMode.Normal
        com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode.TRANSPARENCY -> AmbientSoundMode.Transparency
        com.oppzippy.openscq30.lib.protobuf.AmbientSoundMode.NOISE_CANCELING -> AmbientSoundMode.NoiseCanceling
    }
}

data class AmbientSoundModeCycle(
    val normalMode: Boolean,
    val transparencyMode: Boolean,
    val noiseCancelingMode: Boolean,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.AmbientSoundModeCycle {
        return ambientSoundModeCycle {
            normalMode = this@AmbientSoundModeCycle.normalMode
            transparencyMode = this@AmbientSoundModeCycle.transparencyMode
            noiseCancelingMode = this@AmbientSoundModeCycle.noiseCancelingMode
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.AmbientSoundModeCycle.toKotlin(): AmbientSoundModeCycle {
    return AmbientSoundModeCycle(
        normalMode = normalMode,
        transparencyMode = transparencyMode,
        noiseCancelingMode = noiseCancelingMode,
    )
}

enum class NoiseCancelingMode {
    Indoor,
    Outdoor,
    Transport,
    Custom,
    ;

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode {
        return when (this) {
            Indoor -> com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.INDOOR
            Outdoor -> com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.OUTDOOR
            Transport -> com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.TRANSPORT
            Custom -> com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.CUSTOM
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.toKotlin(): NoiseCancelingMode {
    return when (this) {
        com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.INDOOR -> NoiseCancelingMode.Indoor
        com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.OUTDOOR -> NoiseCancelingMode.Outdoor
        com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.TRANSPORT -> NoiseCancelingMode.Transport
        com.oppzippy.openscq30.lib.protobuf.NoiseCancelingMode.CUSTOM -> NoiseCancelingMode.Custom
    }
}

enum class TransparencyMode {
    FullyTransparent,
    VocalMode,
    ;

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.TransparencyMode {
        return when (this) {
            FullyTransparent -> com.oppzippy.openscq30.lib.protobuf.TransparencyMode.FULLY_TRANSPARENT
            VocalMode -> com.oppzippy.openscq30.lib.protobuf.TransparencyMode.VOCAL_MODE
        }
    }
}

fun com.oppzippy.openscq30.lib.protobuf.TransparencyMode.toKotlin(): TransparencyMode {
    return when (this) {
        com.oppzippy.openscq30.lib.protobuf.TransparencyMode.FULLY_TRANSPARENT -> TransparencyMode.FullyTransparent
        com.oppzippy.openscq30.lib.protobuf.TransparencyMode.VOCAL_MODE -> TransparencyMode.VocalMode
    }
}
