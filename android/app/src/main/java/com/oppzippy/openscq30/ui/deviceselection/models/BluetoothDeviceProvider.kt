package com.oppzippy.openscq30.ui.deviceselection.models

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.app.ActivityCompat
import com.oppzippy.openscq30.lib.SoundcoreDeviceUtils

class BluetoothDeviceProvider(private val context: Context) {
    fun getDevices(): List<BluetoothDeviceModel> {
        val bluetoothManager: BluetoothManager =
            context.getSystemService(BluetoothManager::class.java)
        val adapter: BluetoothAdapter? = bluetoothManager.adapter
        if (adapter != null) {
            if (ActivityCompat.checkSelfPermission(
                    context, Manifest.permission.BLUETOOTH_CONNECT
                ) == PackageManager.PERMISSION_GRANTED
            ) {
                return adapter.bondedDevices.filter {
                    SoundcoreDeviceUtils.isMacAddressSoundcoreDevice(it.address)
                }.map {
                    BluetoothDeviceModel(it.name, it.address)
                }
            } else {
                Log.w("device-selection", "no permission")
            }
        } else {
            Log.w("device-selection", "no bluetooth adapter")
        }
        return listOf()
    }
}