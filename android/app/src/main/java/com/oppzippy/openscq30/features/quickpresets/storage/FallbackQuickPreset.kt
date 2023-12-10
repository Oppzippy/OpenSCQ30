package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Entity
import androidx.room.PrimaryKey
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
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
    val customNoiseCanceling: Int? = null,
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
