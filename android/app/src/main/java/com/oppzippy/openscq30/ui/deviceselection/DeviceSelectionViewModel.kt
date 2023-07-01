package com.oppzippy.openscq30.ui.deviceselection

import androidx.lifecycle.ViewModel
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import javax.inject.Inject

@HiltViewModel
class DeviceSelectionViewModel @Inject constructor(
    private val bluetoothDeviceProvider: BluetoothDeviceProvider,
) : ViewModel() {
    val devices = MutableStateFlow(bluetoothDeviceProvider.getDevices())

    fun refreshDevices() {
        devices.value = bluetoothDeviceProvider.getDevices()
    }
}
