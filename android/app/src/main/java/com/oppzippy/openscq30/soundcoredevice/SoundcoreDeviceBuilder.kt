package com.oppzippy.openscq30.soundcoredevice

import android.bluetooth.BluetoothDevice
import android.content.Context
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.first

@Throws(SecurityException::class)
suspend fun createSoundcoreDevice(
    context: Context, scope: CoroutineScope, bluetoothDevice: BluetoothDevice
): SoundcoreDevice {
    val callbacks = SoundcoreDeviceCallbackHandler()
    val gatt = bluetoothDevice.connectGatt(context, true, callbacks, BluetoothDevice.TRANSPORT_LE)
    gatt.connect()

    val packet = callbacks.packetsFlow.first { it is Packet.StateUpdate } as Packet.StateUpdate
    return SoundcoreDevice(callbacks, scope, SoundcoreDeviceState(packet.packet))
}