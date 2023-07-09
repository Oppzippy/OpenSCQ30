package com.oppzippy.openscq30.features.soundcoredevice.usecases

import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.VolumeAdjustments
import javax.inject.Inject

class ActivateQuickPresetUseCase @Inject constructor(
    private val quickPresetDao: QuickPresetDao,
    private val customProfileDao: CustomProfileDao,
) {
    suspend operator fun invoke(presetId: Int, connectionManager: DeviceConnectionManager) {
        quickPresetDao.get(presetId)?.let { quickPreset ->
            val ambientSoundMode = quickPreset.ambientSoundMode
            val noiseCancelingMode = quickPreset.noiseCancelingMode

            // TODO move quick preset to equalizer configuration logic elsewhere
            val equalizerConfiguration = if (quickPreset.presetEqualizerProfile != null) {
                EqualizerConfiguration(quickPreset.presetEqualizerProfile)
            } else if (quickPreset.customEqualizerProfileName != null) {
                customProfileDao.get(quickPreset.customEqualizerProfileName)?.let {
                    EqualizerConfiguration(VolumeAdjustments(it.values.toByteArray()))
                }
            } else {
                null
            }

            // Set them both in one go if possible to maybe save a packet
            if (ambientSoundMode != null && noiseCancelingMode != null) {
                connectionManager.setSoundMode(ambientSoundMode, noiseCancelingMode)
            } else {
                ambientSoundMode?.let { connectionManager.setAmbientSoundMode(it) }
                noiseCancelingMode?.let { connectionManager.setNoiseCancelingMode(it) }
            }
            equalizerConfiguration?.let {
                connectionManager.setEqualizerConfiguration(it)
            }
        }
    }
}
