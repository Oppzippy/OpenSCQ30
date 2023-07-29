package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Entity
import androidx.room.PrimaryKey
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile

@Entity(
    tableName = "quick_preset",
)
data class QuickPreset(
    @PrimaryKey val id: Int,
    val name: String? = null,
    val ambientSoundMode: AmbientSoundMode? = null,
    val noiseCancelingMode: NoiseCancelingMode? = null,
    val presetEqualizerProfile: PresetEqualizerProfile? = null,
    val customEqualizerProfileName: String? = null,
)
