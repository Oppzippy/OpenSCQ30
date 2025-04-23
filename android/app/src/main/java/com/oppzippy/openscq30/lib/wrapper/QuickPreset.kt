package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("QuickPreset")
data class QuickPreset(val name: String, val isActive: Boolean, val settings: List<QuickPresetField>)

@Serializable
@SerialName("QuickPresetField")
data class QuickPresetField(val settingId: String, val value: Value?)
