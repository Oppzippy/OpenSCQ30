package com.oppzippy.openscq30.features.ui.deviceselection.models

import android.Manifest
import android.annotation.SuppressLint
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.app.ActivityCompat
import com.oppzippy.openscq30.lib.SoundcoreDeviceUtils

class BluetoothDeviceProvider(private val context: Context) {
    /**
     * The caller is responsible for checking for bluetooth permission
     */
    @SuppressLint("MissingPermission")
    fun getDevices(): List<BluetoothDeviceModel> {
        val bluetoothManager: BluetoothManager =
            context.getSystemService(BluetoothManager::class.java)
        val adapter: BluetoothAdapter? = bluetoothManager.adapter
        if (adapter != null) {
            return adapter.bondedDevices.filter {
                SoundcoreDeviceUtils.isMacAddressSoundcoreDevice(it.address)
            }.map {
                BluetoothDeviceModel(it.name, it.address)
            }
        } else {
            Log.w("device-selection", "no bluetooth adapter")
        }
        return listOf()
    }
}