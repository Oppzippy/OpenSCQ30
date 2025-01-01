package com.oppzippy.openscq30.features.soundcoredevice.usecases

import android.util.Log
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetRepository
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.lib.extensions.resources.toEqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import javax.inject.Inject

class ActivateQuickPresetUseCase @Inject constructor(
    private val quickPresetRepository: QuickPresetRepository,
    private val customProfileDao: CustomProfileDao,
) {
    suspend operator fun invoke(presetId: Int, connectionManager: DeviceConnectionManager) {
        val connectionStatus = connectionManager.connectionStatusFlow.value
        if (connectionStatus !is ConnectionStatus.Connected) return
        val model = connectionStatus.device.stateFlow.value.model
        if (model == null) {
            Log.w("ActivateQuickPresetUseCase", "device model is null")
            return
        }

        quickPresetRepository.getForDevice(model).getOrNull(presetId)?.let { quickPreset ->
            // TODO move quick preset to equalizer configuration logic elsewhere
            val equalizerConfiguration = if (quickPreset.presetEqualizerProfile != null) {
                quickPreset.presetEqualizerProfile.toEqualizerConfiguration()
            } else if (quickPreset.customEqualizerProfileName != null) {
                customProfileDao.get(quickPreset.customEqualizerProfileName)?.let {
                    EqualizerConfiguration(
                        volumeAdjustments = it.getVolumeAdjustments(),
                    )
                }
            } else {
                null
            }

            connectionStatus.device.stateFlow.value.soundModes?.let { prevSoundModes ->
                val newSoundModes = prevSoundModes.copy(
                    ambientSoundMode = quickPreset.ambientSoundMode
                        ?: prevSoundModes.ambientSoundMode,
                    noiseCancelingMode =
                    quickPreset.noiseCancelingMode ?: prevSoundModes.noiseCancelingMode,
                    transparencyMode = quickPreset.transparencyMode
                        ?: prevSoundModes.transparencyMode,
                    customNoiseCanceling = quickPreset.customNoiseCanceling?.toUByte()
                        ?: prevSoundModes.customNoiseCanceling,

                )
                connectionManager.setSoundModes(newSoundModes)
            }
            equalizerConfiguration?.let {
                connectionManager.setEqualizerConfiguration(it)
            }
        }
    }
}
