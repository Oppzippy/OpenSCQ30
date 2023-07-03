package com.oppzippy.openscq30.features.bluetoothdeviceprovider

import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Context
import android.util.Log
import androidx.annotation.RequiresPermission
import com.oppzippy.openscq30.lib.SoundcoreDeviceUtils

class BluetoothDeviceProviderImpl(private val context: Context) : BluetoothDeviceProvider {
    /**
     * The caller is responsible for checking for bluetooth permission
     */
    @RequiresPermission(value = "android.permission.BLUETOOTH_CONNECT")
    override fun getDevices(): List<BluetoothDevice> {
        val bluetoothManager: BluetoothManager =
            context.getSystemService(BluetoothManager::class.java)
        val adapter: BluetoothAdapter? = bluetoothManager.adapter
        if (adapter != null) {
            return adapter.bondedDevices.filter {
                SoundcoreDeviceUtils.isMacAddressSoundcoreDevice(it.address)
            }.map {
                BluetoothDevice(it.name, it.address)
            }
        } else {
            Log.w("device-selection", "no bluetooth adapter")
        }
        return listOf()
    }
}
