package com.oppzippy.openscq30.lib.wrapper

import kotlinx.serialization.Serializable

@Serializable
data class QuickPreset(val name: String, val fields: List<QuickPresetField>)

@Serializable
data class QuickPresetField(val settingId: String, val value: Value, val isEnabled: Boolean)
