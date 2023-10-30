package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.content.Context
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject

class BluetoothDeviceFinderImpl @Inject constructor(@ApplicationContext private val context: Context) :
    BluetoothDeviceFinder {
    @SuppressLint("MissingPermission")
    override fun findByMacAddress(
        macAddress: String,
    ): BluetoothDevice? {
        val bluetoothManager: BluetoothManager =
            context.getSystemService(BluetoothManager::class.java)
        return bluetoothManager.adapter.bondedDevices.find { it.address == macAddress }
    }
}
