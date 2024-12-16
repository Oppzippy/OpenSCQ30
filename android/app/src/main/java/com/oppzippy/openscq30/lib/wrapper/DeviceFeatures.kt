package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.protobuf.AvailableSoundModes as ProtobufAvailableSoundModes
import com.oppzippy.openscq30.lib.protobuf.DeviceFeatures as ProtobufDeviceFeatures
import com.oppzippy.openscq30.lib.protobuf.NoiseCancelingModeType as ProtobufNoiseCancelingModeType
import com.oppzippy.openscq30.lib.protobuf.TransparencyModeType as ProtobufTransparencyModeType
import com.oppzippy.openscq30.lib.protobuf.availableSoundModes
import com.oppzippy.openscq30.lib.protobuf.availableSoundModesOrNull
import com.oppzippy.openscq30.lib.protobuf.deviceFeatures
import com.oppzippy.openscq30.lib.protobuf.dynamicRangeCompressionMinFirmwareVersionOrNull

data class DeviceFeatures(
    val availableSoundModes: AvailableSoundModes?,
    val hasHearId: Boolean,
    val numEqualizerChannels: Int,
    val numEqualizerBands: Int,
    val hasDynamicRangeCompression: Boolean,
    val hasButtonConfiguration: Boolean,
    val hasWearDetection: Boolean,
    val hasTouchTone: Boolean,
    val hasAutoPowerOff: Boolean,
    val dynamicRangeCompressionMinFirmwareVersion: FirmwareVersion?,
) {
    fun toProtobuf(): ProtobufDeviceFeatures = deviceFeatures {
        this@DeviceFeatures.availableSoundModes?.let { availableSoundModes = it.toProtobuf() }
        hasHearId = this@DeviceFeatures.hasHearId
        numEqualizerChannels = this@DeviceFeatures.numEqualizerChannels
        numEqualizerBands = this@DeviceFeatures.numEqualizerBands
        hasDynamicRangeCompression = this@DeviceFeatures.hasDynamicRangeCompression
        hasButtonConfiguration = this@DeviceFeatures.hasButtonConfiguration
        hasWearDetection = this@DeviceFeatures.hasWearDetection
        hasTouchTone = this@DeviceFeatures.hasTouchTone
        hasAutoPowerOff = this@DeviceFeatures.hasAutoPowerOff
        this@DeviceFeatures.dynamicRangeCompressionMinFirmwareVersion?.let {
            dynamicRangeCompressionMinFirmwareVersion = it.toProtobuf()
        }
    }
}

fun ProtobufDeviceFeatures.toKotlin(): DeviceFeatures = DeviceFeatures(
    availableSoundModes = availableSoundModesOrNull?.toKotlin(),
    hasHearId = hasHearId,
    numEqualizerChannels = numEqualizerChannels,
    numEqualizerBands = numEqualizerBands,
    hasDynamicRangeCompression = hasDynamicRangeCompression,
    hasButtonConfiguration = hasButtonConfiguration,
    hasWearDetection = hasWearDetection,
    hasTouchTone = hasTouchTone,
    hasAutoPowerOff = hasAutoPowerOff,
    dynamicRangeCompressionMinFirmwareVersion = dynamicRangeCompressionMinFirmwareVersionOrNull?.toKotlin(),
)

data class AvailableSoundModes(
    val ambientSoundModes: List<AmbientSoundMode>,
    val transparencyModes: List<TransparencyMode>,
    val noiseCancelingModes: List<NoiseCancelingMode>,
    val customNoiseCanceling: Boolean,
) {
    fun toProtobuf(): ProtobufAvailableSoundModes = availableSoundModes {
        ambientSoundModes.addAll(this@AvailableSoundModes.ambientSoundModes.map { it.toProtobuf() })
        transparencyModes.addAll(this@AvailableSoundModes.transparencyModes.map { it.toProtobuf() })
        noiseCancelingModes.addAll(this@AvailableSoundModes.noiseCancelingModes.map { it.toProtobuf() })
        customNoiseCanceling = this@AvailableSoundModes.customNoiseCanceling
    }
}

fun ProtobufAvailableSoundModes.toKotlin(): AvailableSoundModes = AvailableSoundModes(
    ambientSoundModes = ambientSoundModesList.map { it.toKotlin() },
    transparencyModes = transparencyModesList.map { it.toKotlin() },
    noiseCancelingModes = noiseCancelingModesList.map { it.toKotlin() },
    customNoiseCanceling = customNoiseCanceling,
)

enum class NoiseCancelingModeType {
    None,
    Basic,
    Custom,
    ;

    fun toProtobuf(): ProtobufNoiseCancelingModeType = when (this) {
        None -> ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_NONE
        Basic -> ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_BASIC
        Custom -> ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_CUSTOM
    }
}

fun ProtobufNoiseCancelingModeType.toKotlin(): NoiseCancelingModeType = when (this) {
    ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_NONE -> NoiseCancelingModeType.None
    ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_BASIC -> NoiseCancelingModeType.Basic
    ProtobufNoiseCancelingModeType.NOISE_CANCELING_MODE_CUSTOM -> NoiseCancelingModeType.Custom
}

enum class TransparencyModeType {
    Basic,
    Custom,
    ;

    fun toProtobuf(): ProtobufTransparencyModeType = when (this) {
        Basic -> ProtobufTransparencyModeType.TRANSPARENCY_MODE_BASIC
        Custom -> ProtobufTransparencyModeType.TRANSPARENCY_MODE_CUSTOM
    }
}

fun ProtobufTransparencyModeType.toKotlin(): TransparencyModeType = when (this) {
    ProtobufTransparencyModeType.TRANSPARENCY_MODE_BASIC -> TransparencyModeType.Basic
    ProtobufTransparencyModeType.TRANSPARENCY_MODE_CUSTOM -> TransparencyModeType.Custom
}
