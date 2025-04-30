package com.oppzippy.openscq30.lib.wrapper

import android.os.Parcelable
import kotlinx.parcelize.Parcelize
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Parcelize
@Serializable
@SerialName("ConnectionDescriptor")
data class ConnectionDescriptor(val name: String, val macAddress: String) : Parcelable

@Serializable
@SerialName("ConnectionStatus")
enum class ConnectionStatus {
    @SerialName("connected")
    Connected,

    @SerialName("disconnected")
    Disconnected,
}

@Serializable
@SerialName("DeviceDescriptor")
data class DeviceDescriptor(val name: String, val macAddress: String, val model: String)
