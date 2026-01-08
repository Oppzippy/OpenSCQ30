package com.oppzippy.openscq30.lib.wrapper

import android.os.Parcelable
import kotlinx.parcelize.Parcelize
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Parcelize
@Serializable
@SerialName("value")
sealed class Value : Parcelable {
    @Parcelize
    @Serializable
    @SerialName("bool")
    data class BoolValue(val value: Boolean) : Value()

    @Parcelize
    @Serializable
    @SerialName("u16")
    data class U16Value(val value: UShort) : Value()

    @Parcelize
    @Serializable
    @SerialName("u16Vec")
    data class U16VecValue(val value: List<UShort>) : Value()

    @Parcelize
    @Serializable
    @SerialName("optionalU16")
    data class OptionalU16Value(val value: UShort?) : Value()

    @Parcelize
    @Serializable
    @SerialName("i16Vec")
    data class I16VecValue(val value: List<Short>) : Value()

    @Parcelize
    @Serializable
    @SerialName("i32")
    data class I32Value(val value: Int) : Value()

    @Parcelize
    @Serializable
    @SerialName("string")
    data class StringValue(val value: String) : Value()

    @Parcelize
    @Serializable
    @SerialName("stringVec")
    data class StringVecValue(val value: List<String>) : Value()

    @Parcelize
    @Serializable
    @SerialName("optionalString")
    data class OptionalStringValue(val value: String?) : Value()

    @Parcelize
    @Serializable
    @SerialName("modifiableSelectCommand")
    data class ModifiableSelectCommand(val value: ModifiableSelectCommandInner) : Value()
}

@Parcelize
@Serializable
sealed class ModifiableSelectCommandInner : Parcelable {
    @Parcelize
    @Serializable
    @SerialName("add")
    data class Add(val name: String) : ModifiableSelectCommandInner()

    @Parcelize
    @Serializable
    @SerialName("remove")
    data class Remove(val name: String) : ModifiableSelectCommandInner()

    fun toValue(): Value.ModifiableSelectCommand = Value.ModifiableSelectCommand(this)
}

fun Boolean.toValue(): Value.BoolValue = Value.BoolValue(this)

fun UShort.toValue(): Value.U16Value = Value.U16Value(this)

fun List<UShort>.toValue(): Value.U16VecValue = Value.U16VecValue(this)

fun UShort?.toValue(): Value.OptionalU16Value = Value.OptionalU16Value(this)

fun List<Short>.toValue(): Value.I16VecValue = Value.I16VecValue(this)

fun Int.toValue(): Value.I32Value = Value.I32Value(this)

fun String.toValue(): Value.StringValue = Value.StringValue(this)

fun List<String>.toValue(): Value.StringVecValue = Value.StringVecValue(this)

fun String?.toValue(): Value.OptionalStringValue = Value.OptionalStringValue(this)
