package com.oppzippy.openscq30.soundcoredevice

import android.Manifest
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import androidx.lifecycle.LifecycleCoroutineScope
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import dagger.hilt.android.qualifiers.ActivityContext
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.first
import javax.inject.Inject

class SoundcoreDeviceFactory @Inject constructor(
    @ActivityContext private val context: Context,
    private val scope: LifecycleCoroutineScope,
) {
    @Throws(SecurityException::class)
    suspend fun createSoundcoreDevice(macAddress: String): SoundcoreDevice? {
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
