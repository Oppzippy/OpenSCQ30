package com.oppzippy.openscq30.features.bluetoothdeviceprovider

interface BluetoothDeviceProvider{
    fun getDevices(): List<BluetoothDevice>
}
