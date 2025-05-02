package com.oppzippy.openscq30.lib.wrapper

import android.os.Parcelable
import kotlinx.parcelize.Parcelize
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Parcelize
@Serializable
data class ConnectionDescriptor(val name: String, val macAddress: String) : Parcelable

@Serializable
enum class ConnectionStatus {
    @SerialName("connected")
    Connected,

    @SerialName("disconnected")
    Disconnected,
}

@Serializable
data class DeviceDescriptor(val name: String, val macAddress: String, val model: String)
