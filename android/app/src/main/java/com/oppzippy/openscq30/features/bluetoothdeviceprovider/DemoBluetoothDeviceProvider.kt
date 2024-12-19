package com.oppzippy.openscq30.features.bluetoothdeviceprovider

class DemoBluetoothDeviceProvider : BluetoothDeviceProvider {
    override fun getDevices(): List<BluetoothDevice> =
        listOf(BluetoothDevice("Demo Q30", "00:00:00:00:00:00", isAssociated = true))
}
