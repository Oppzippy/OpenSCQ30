package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Entity
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import java.util.UUID

@Entity(
    tableName = "quick_preset",
    primaryKeys = ["deviceBleServiceUuid", "index"],
)
data class QuickPreset(
    val deviceBleServiceUuid: UUID,
    val index: Int,
    val name: String? = null,
    val ambientSoundMode: AmbientSoundMode? = null,
    val noiseCancelingMode: NoiseCancelingMode? = null,
    val transparencyMode: TransparencyMode? = null,
    val customNoiseCanceling: CustomNoiseCanceling? = null,
    val presetEqualizerProfile: PresetEqualizerProfile? = null,
    val customEqualizerProfileName: String? = null,
)
