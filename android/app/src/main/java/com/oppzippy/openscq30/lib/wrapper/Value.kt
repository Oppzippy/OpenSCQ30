package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("Value")
sealed class Value {
    @Serializable
    @SerialName("Bool")
    data class BoolValue(val value: Boolean) : Value()

    @Serializable
    @SerialName("U16")
    data class U16Value(val value: UShort) : Value()

    @Serializable
    @SerialName("U16Vec")
    data class U16VecValue(val value: List<UShort>) : Value()

    @Serializable
    @SerialName("OptionalU16")
    data class OptionalU16Value(val value: UShort?) : Value()

    @Serializable
    @SerialName("I16Vec")
    data class I16VecValue(val value: List<Short>) : Value()

    @Serializable
    @SerialName("I32")
    data class I32Value(val value: Int) : Value()

    @Serializable
    @SerialName("String")
    data class StringValue(val value: String) : Value()

    @Serializable
    @SerialName("StringVec")
    data class StringVecValue(val value: List<String>) : Value()

    @Serializable
    @SerialName("OptionalString")
    data class OptionalStringValue(val value: String?) : Value()

    @Serializable
    @SerialName("ModifiableSelectCommand")
    data class ModifiableSelectCommand(val value: ModifiableSelectCommandInner) : Value()
}

@Serializable
@SerialName("ModifiableSelectCommand")
sealed class ModifiableSelectCommandInner {
    @Serializable
    @SerialName("Add")
    data class Add(val name: String) : ModifiableSelectCommandInner()

    @Serializable
    @SerialName("Remove")
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
