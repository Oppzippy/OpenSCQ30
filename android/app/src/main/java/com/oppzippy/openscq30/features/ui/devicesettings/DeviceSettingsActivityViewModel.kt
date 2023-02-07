package com.oppzippy.openscq30.features.ui.devicesettings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceBox
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject

@HiltViewModel
class DeviceSettingsActivityViewModel @Inject constructor(val soundcoreDeviceBox: SoundcoreDeviceBox) :
    ViewModel() {
    suspend fun setMacAddress(macAddress: String) {
        soundcoreDeviceBox.setDevice(macAddress, viewModelScope)
    }
}