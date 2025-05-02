package com.oppzippy.openscq30.lib.wrapper

import android.os.Parcelable
import kotlinx.parcelize.Parcelize
import kotlinx.serialization.Serializable

@Parcelize
@Serializable
data class PairedDevice(val macAddress: String, val model: String, val isDemo: Boolean) : Parcelable
