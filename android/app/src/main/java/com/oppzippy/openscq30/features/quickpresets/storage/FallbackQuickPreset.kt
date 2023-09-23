package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Entity
import androidx.room.PrimaryKey
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import java.util.UUID

@Entity(
    tableName = "fallback_quick_preset",
)
data class FallbackQuickPreset(
    @PrimaryKey val index: Int,
    val name: String? = null,
    val ambientSoundMode: AmbientSoundMode? = null,
    val noiseCancelingMode: NoiseCancelingMode? = null,
    val transparencyMode: TransparencyMode? = null,
    val customNoiseCanceling: CustomNoiseCanceling? = null,
    val presetEqualizerProfile: PresetEqualizerProfile? = null,
    val customEqualizerProfileName: String? = null,
) {
    fun toQuickPreset(deviceBleServiceUuid: UUID): QuickPreset {
        return QuickPreset(
            deviceBleServiceUuid = deviceBleServiceUuid,
            index,
            name,
            ambientSoundMode,
            noiseCancelingMode,
            transparencyMode,
            customNoiseCanceling,
            presetEqualizerProfile,
        )
    }
}
