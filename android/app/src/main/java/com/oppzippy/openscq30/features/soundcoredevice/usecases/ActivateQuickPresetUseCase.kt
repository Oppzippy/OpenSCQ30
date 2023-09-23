package com.oppzippy.openscq30.features.soundcoredevice.usecases

import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetRepository
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.SoundModes
import javax.inject.Inject

class ActivateQuickPresetUseCase @Inject constructor(
    private val quickPresetRepository: QuickPresetRepository,
    private val customProfileDao: CustomProfileDao,
) {
    suspend operator fun invoke(presetId: Int, connectionManager: DeviceConnectionManager) {
        val connectionStatus = connectionManager.connectionStatusFlow.value
        if (connectionStatus !is ConnectionStatus.Connected) return

        quickPresetRepository.getForDevice(connectionStatus.device.bleServiceUuid)
            .getOrNull(presetId)?.let { quickPreset ->
                val ambientSoundMode = quickPreset.ambientSoundMode
                val noiseCancelingMode = quickPreset.noiseCancelingMode

                // TODO move quick preset to equalizer configuration logic elsewhere
                val equalizerConfiguration = if (quickPreset.presetEqualizerProfile != null) {
                    EqualizerConfiguration(quickPreset.presetEqualizerProfile)
                } else if (quickPreset.customEqualizerProfileName != null) {
                    customProfileDao.get(quickPreset.customEqualizerProfileName)?.let {
                        EqualizerConfiguration(it.getVolumeAdjustments())
                    }
                } else {
                    null
                }

                connectionStatus.device.state.soundModes?.let { prevSoundModes ->
                    val newSoundModes = SoundModes(
                        ambientSoundMode ?: prevSoundModes.ambientSoundMode(),
                        noiseCancelingMode ?: prevSoundModes.noiseCancelingMode(),
                        prevSoundModes.transparencyMode(),
                        prevSoundModes.customNoiseCanceling(),
                    )
                    connectionManager.setSoundModes(newSoundModes)
                }
                equalizerConfiguration?.let {
                    connectionManager.setEqualizerConfiguration(it)
                }
            }
    }
}
