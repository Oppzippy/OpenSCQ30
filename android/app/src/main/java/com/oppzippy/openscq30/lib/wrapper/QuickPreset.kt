package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.Serializable

@Serializable
data class QuickPreset(val name: String, val isActive: Boolean, val settings: List<QuickPresetField>)

@Serializable
data class QuickPresetField(val settingId: String, val value: Value?)
