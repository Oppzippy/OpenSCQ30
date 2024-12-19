package com.oppzippy.openscq30.features.bluetoothdeviceprovider

import android.bluetooth.BluetoothManager
import android.companion.CompanionDeviceManager
import android.content.Context
import androidx.annotation.RequiresPermission
import com.oppzippy.openscq30.lib.bindings.isMacAddressSoundcoreDevice
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject

class BluetoothDeviceProviderImpl @Inject constructor(@ApplicationContext private val context: Context) :
    BluetoothDeviceProvider {
    @RequiresPermission(value = "android.permission.BLUETOOTH_CONNECT")
    override fun getDevices(): List<BluetoothDevice> {
        val bluetoothManager = context.getSystemService(BluetoothManager::class.java)
        val deviceManager = context.getSystemService(CompanionDeviceManager::class.java)
        val boundDevices = bluetoothManager.adapter.bondedDevices.filter { isMacAddressSoundcoreDevice(it.address) }
        val associatedDevices = deviceManager.associations.toHashSet()
        return boundDevices.map { device ->
            BluetoothDevice(
                name = device.name,
                address = device.address,
                isAssociated = associatedDevices.contains(device.address),
            )
        }
    }
}
