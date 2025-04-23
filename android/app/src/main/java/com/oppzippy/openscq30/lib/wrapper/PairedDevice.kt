package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("PairedDevice")
data class PairedDevice(
    val name: String,
    val macAddress: String,
    val model: String,
)

