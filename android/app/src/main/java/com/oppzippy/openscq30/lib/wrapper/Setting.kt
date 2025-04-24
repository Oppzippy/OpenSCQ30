package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("Setting")
sealed class Setting {
    @Serializable
    @SerialName("Toggle")
    data class ToggleSetting(val value: Boolean) : Setting()

    @Serializable
    @SerialName("I32Range")
    data class I32RangeSetting(val setting: Range<Int>, val value: Int) : Setting()

    @Serializable
    @SerialName("Select")
    data class SelectSetting(val setting: Select, val value: String) : Setting()

    @Serializable
    @SerialName("OptionalSelect")
    data class OptionalSelectSetting(val setting: Select, val value: String?) : Setting()

    @Serializable
    @SerialName("ModifiableSelect")
    data class ModifiableSelectSetting(val setting: Select, val value: String?) : Setting()

    @Serializable
    @SerialName("Equalizer")
    data class EqualizerSetting(val setting: Equalizer, val value: List<Short>) : Setting()

    @Serializable
    @SerialName("Information")
    data class InformationSetting(val text: String, val translatedText: String) : Setting()
}

@Serializable
@SerialName("Range")
data class Range<T>(val from: T, val to: T, val step: T)

@Serializable
@SerialName("Select")
data class Select(val options: List<String>, val localizedOptions: List<String>)

@Serializable
@SerialName("Equalizer")
data class Equalizer(val bandHz: List<UShort>, val fractionDigits: Short, val min: Short, val max: Short)
