package com.oppzippy.openscq30.soundcoredevice

import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.content.Context
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.first

class SoundcoreDeviceFactory(private val context: Context) {
    @Throws(SecurityException::class)
    suspend fun createSoundcoreDevice(macAddress: String, scope: CoroutineScope): SoundcoreDevice? {
        val bluetoothManager: BluetoothManager =
            context.getSystemService(BluetoothManager::class.java)
        val bluetoothDevice =
            bluetoothManager.adapter.bondedDevices.find { it.address == macAddress } ?: return null

        val callbacks = SoundcoreDeviceCallbackHandler()
        val gatt =
            bluetoothDevice.connectGatt(context, false, callbacks, BluetoothDevice.TRANSPORT_LE)
        gatt.connect()

        val packet = callbacks.packetsFlow.first { it is Packet.StateUpdate } as Packet.StateUpdate
        return SoundcoreDevice(gatt, callbacks, scope, SoundcoreDeviceState(packet.packet))
    }
}
