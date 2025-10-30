package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("Setting")
sealed class Setting {
    abstract fun toValue(): Value

    @Serializable
    @SerialName("toggle")
    data class ToggleSetting(val value: Boolean) : Setting() {
        override fun toValue() = value.toValue()
    }

    @Serializable
    @SerialName("i32Range")
    data class I32RangeSetting(val setting: Range<Int>, val value: Int) : Setting() {
        override fun toValue() = value.toValue()
    }

    @Serializable
    @SerialName("select")
    data class SelectSetting(val setting: Select, val value: String) : Setting() {
        override fun toValue() = value.toValue()
    }

    @Serializable
    @SerialName("optionalSelect")
    data class OptionalSelectSetting(val setting: Select, val value: String?) : Setting() {
        override fun toValue() = value.toValue()
    }

    @Serializable
    @SerialName("modifiableSelect")
    data class ModifiableSelectSetting(val setting: Select, val value: String?) : Setting() {
        override fun toValue() = value.toValue()
    }

    @Serializable
    @SerialName("multiSelect")
    data class MultiSelectSetting(val setting: Select, val values: List<String>) : Setting() {
        override fun toValue() = values.toValue()
    }

    @Serializable
    @SerialName("equalizer")
    data class EqualizerSetting(val setting: Equalizer, val value: List<Short>) : Setting() {
        override fun toValue() = value.toValue()
    }

    @Serializable
    @SerialName("information")
    data class InformationSetting(val value: String, val translatedValue: String) : Setting() {
        override fun toValue() = value.toValue()
    }

    @Serializable
    @SerialName("importString")
    data class ImportStringSetting(val confirmationMessage: String?) : Setting() {
        override fun toValue() = "".toValue()
    }

    @Serializable
    @SerialName("action")
    class Action : Setting() {
        override fun toValue() = false.toValue()
    }
}

@Serializable
data class Range<T>(val start: T, val end: T, val step: T)

@Serializable
data class Select(val options: List<String>, val localizedOptions: List<String>)

@Serializable
data class Equalizer(val bandHz: List<UShort>, val fractionDigits: Short, val min: Short, val max: Short)
