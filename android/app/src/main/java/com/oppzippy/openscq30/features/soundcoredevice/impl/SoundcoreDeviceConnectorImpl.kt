package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothDevice
import android.content.Context
import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.lib.bindings.ManualConnection
import com.oppzippy.openscq30.lib.bindings.newSoundcoreDevice
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.launch
import kotlinx.coroutines.withTimeout
import java.util.concurrent.TimeoutException
import kotlin.time.Duration.Companion.seconds

class SoundcoreDeviceConnectorImpl(
    private val context: Context,
    private val deviceFinder: BluetoothDeviceFinder,
) : SoundcoreDeviceConnector {
    @Throws(SecurityException::class)
    override suspend fun connectToSoundcoreDevice(
        macAddress: String,
        coroutineScope: CoroutineScope,
    ): SoundcoreDevice? {
        val bluetoothDevice = deviceFinder.findByMacAddress(macAddress) ?: return null
        val callbacks =
            SoundcoreDeviceCallbackHandler(context = context, coroutineScope = coroutineScope)
        val gatt =
            bluetoothDevice.connectGatt(context, false, callbacks, BluetoothDevice.TRANSPORT_LE)
        if (!gatt.connect()) {
            Log.d("SoundcoreDeviceFactoryImpl", "gatt connect failed")
            return null
        }

        try {
            withTimeout(4.seconds) {
                callbacks.waitUntilReady()
            }
        } catch (ex: TimeoutCancellationException) {
            throw TimeoutException("Timeout waiting for GATT services").initCause(ex)
        }

        val serviceUuid = callbacks.serviceUuid.value ?: return null
        val connection = ManualConnection(
            name = bluetoothDevice.name,
            macAddress = bluetoothDevice.address,
            serviceUuid = serviceUuid,
            connectionWriter = callbacks,
        )
        callbacks.setManualConnection(connection)

        val nativeDevice = newSoundcoreDevice(connection)
        val soundcoreDevice = SoundcoreDevice(
            name = nativeDevice.name(),
            macAddress = nativeDevice.macAddress(),
            bleServiceUuid = nativeDevice.serviceUuid(),
            cleanUp = {
                callbacks.close()
                gatt.disconnect()
                gatt.close()
            },
            nativeDevice = nativeDevice,
            coroutineScope = coroutineScope,
            initialState = nativeDevice.state(),
        )

        // SoundcoreDevice and SoundcoreDeviceCallbackHandler are intentionally unaware of each other,
        // so connecting isDisconnected to SoundcoreDevice's close must be done outside of either of
        // the two classes.
        coroutineScope.launch {
            callbacks.isDisconnected.collect { isDisconnected ->
                if (isDisconnected) {
                    soundcoreDevice.close()
                }
            }
        }

        return soundcoreDevice
    }
}
