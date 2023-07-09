package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Entity
import androidx.room.PrimaryKey
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.lib.PresetEqualizerProfile

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
