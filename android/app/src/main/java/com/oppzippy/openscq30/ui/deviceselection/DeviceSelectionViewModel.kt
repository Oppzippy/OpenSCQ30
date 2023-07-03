package com.oppzippy.openscq30.ui.deviceselection

import android.app.Application
import android.content.pm.PackageManager
import android.os.Build
import androidx.lifecycle.AndroidViewModel
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import javax.inject.Inject

@HiltViewModel
class DeviceSelectionViewModel @Inject constructor(
    private val application: Application,
    private val bluetoothDeviceProvider: BluetoothDeviceProvider,
) : AndroidViewModel(application) {
    val devices = MutableStateFlow(getDevices())

    fun refreshDevices() {
        devices.value = getDevices()
    }

    private fun getDevices(): List<BluetoothDevice> {
        val hasBluetoothPermission = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            application.checkSelfPermission(android.Manifest.permission.BLUETOOTH_CONNECT) == PackageManager.PERMISSION_GRANTED
        } else {
            application.checkSelfPermission(android.Manifest.permission.BLUETOOTH) == PackageManager.PERMISSION_GRANTED
        }
        return if (hasBluetoothPermission) {
            bluetoothDeviceProvider.getDevices()
        } else {
            emptyList()
        }
    }
}
