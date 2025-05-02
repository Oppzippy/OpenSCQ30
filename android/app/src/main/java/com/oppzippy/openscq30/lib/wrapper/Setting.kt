package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("Setting")
sealed class Setting {
    @Serializable
    @SerialName("toggle")
    data class ToggleSetting(val value: Boolean) : Setting()

    @Serializable
    @SerialName("i32Range")
    data class I32RangeSetting(val setting: Range<Int>, val value: Int) : Setting()

    @Serializable
    @SerialName("select")
    data class SelectSetting(val setting: Select, val value: String) : Setting()

    @Serializable
    @SerialName("optionalSelect")
    data class OptionalSelectSetting(val setting: Select, val value: String?) : Setting()

    @Serializable
    @SerialName("modifiableSelect")
    data class ModifiableSelectSetting(val setting: Select, val value: String?) : Setting()

    @Serializable
    @SerialName("equalizer")
    data class EqualizerSetting(val setting: Equalizer, val value: List<Short>) : Setting()

    @Serializable
    @SerialName("information")
    data class InformationSetting(val text: String, val translatedText: String) : Setting()
}

@Serializable
data class Range<T>(val from: T, val to: T, val step: T)

@Serializable
data class Select(val options: List<String>, val localizedOptions: List<String>)

@Serializable
data class Equalizer(val bandHz: List<UShort>, val fractionDigits: Short, val min: Short, val max: Short)
