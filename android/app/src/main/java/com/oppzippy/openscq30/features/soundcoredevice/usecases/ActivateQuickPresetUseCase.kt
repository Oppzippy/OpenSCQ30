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
    suspend operator fun invoke(presetNumber: Int, connectionManager: DeviceConnectionManager) {
        quickPresetDao.get(presetNumber)?.let { quickPreset ->
            val ambientSoundMode = quickPreset.ambientSoundMode
            val noiseCancelingMode = quickPreset.noiseCancelingMode
            val equalizerConfiguration = quickPreset.equalizerProfileName?.let {
                customProfileDao.get(it)
            }?.let {
                EqualizerConfiguration(VolumeAdjustments(it.values.toByteArray()))
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
