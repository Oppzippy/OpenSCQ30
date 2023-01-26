package com.oppzippy.openscq30.ui.devicesettings.composables.general

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.devicesettings.SoundcoreDeviceBox
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import javax.inject.Inject

@OptIn(ExperimentalCoroutinesApi::class)
@HiltViewModel
class GeneralSettingsViewModel @Inject constructor(private val deviceBox: SoundcoreDeviceBox) :
    ViewModel() {
    var ambientSoundMode: StateFlow<AmbientSoundMode>? = null
    var noiseCancelingMode: StateFlow<NoiseCancelingMode>? = null

    init {
        viewModelScope.launch {
            deviceBox.device.collectLatest { device ->
                if (device == null) {
                    throw IllegalStateException("device must not be null")
                }
                ambientSoundMode = device.stateFlow.mapLatest { it.ambientSoundMode() }
                    .stateIn(viewModelScope, SharingStarted.Eagerly, device.state.ambientSoundMode())
                noiseCancelingMode = device.stateFlow.mapLatest { it.noiseCancelingMode() }
                    .stateIn(viewModelScope, SharingStarted.Eagerly, device.state.noiseCancelingMode())
            }
        }
    }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        deviceBox.device.value?.let { device ->
            device.setSoundMode(ambientSoundMode, device.state.noiseCancelingMode())
        }
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        deviceBox.device.value?.let { device ->
            device.setSoundMode(device.state.ambientSoundMode(), noiseCancelingMode)
        }
    }
}