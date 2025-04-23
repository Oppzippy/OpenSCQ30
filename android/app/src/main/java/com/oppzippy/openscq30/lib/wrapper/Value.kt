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
    data class I32Value(val value: List<Int>) : Value()

    @Serializable
    @SerialName("String")
    data class StringValue(val value: String) : Value()

    @Serializable
    @SerialName("StringVec")
    data class StringVecValue(val value: List<String>) : Value()

    @Serializable
    @SerialName("OptionalString")
    data class OptionalStringValue(val value: String?) : Value()

}
