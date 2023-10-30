package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothDevice

interface BluetoothDeviceFinder {
    fun findByMacAddress(
        macAddress: String,
    ): BluetoothDevice?
}
