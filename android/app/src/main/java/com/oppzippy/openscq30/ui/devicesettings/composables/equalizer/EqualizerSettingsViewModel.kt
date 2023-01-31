package com.oppzippy.openscq30.ui.devicesettings.composables.equalizer

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.soundcoredevice.contentEquals
import com.oppzippy.openscq30.ui.devicesettings.SoundcoreDeviceBox
import com.oppzippy.openscq30.ui.devicesettings.models.EqualizerProfile
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import javax.inject.Inject

@OptIn(ExperimentalCoroutinesApi::class, FlowPreview::class)
@HiltViewModel
class EqualizerSettingsViewModel @Inject constructor(private val deviceBox: SoundcoreDeviceBox) :
    ViewModel() {
    val displayedEqualizerConfiguration: MutableStateFlow<EqualizerConfiguration?> =
        MutableStateFlow(null)

    init {
        viewModelScope.launch {
            deviceBox.device.collectLatest { device ->
                if (device == null) {
                    throw IllegalStateException("device must not be null")
                }
                device.stateFlow.mapLatest {
                    it.equalizerConfiguration()
                }.distinctUntilChanged { old, new -> old.contentEquals(new) }.collectLatest {
                    displayedEqualizerConfiguration.value = EqualizerConfiguration.fromRust(it)
                }
            }
        }

        viewModelScope.launch {
            displayedEqualizerConfiguration
                .debounce(500).collectLatest {
                    if (it != null) {
                        deviceBox.device.value?.setEqualizerConfiguration(it.toRust())
                    }
                }
        }
    }

    fun setEqualizerConfiguration(profile: EqualizerProfile, values: ByteArray) {
        // Values match the display, not the profile. Creating the rust EqualizerConfiguration first
        // will use proper values.
        val configuration = EqualizerConfiguration.fromRust(profile.toEqualizerConfiguration(values))
        displayedEqualizerConfiguration.value = configuration
    }
}