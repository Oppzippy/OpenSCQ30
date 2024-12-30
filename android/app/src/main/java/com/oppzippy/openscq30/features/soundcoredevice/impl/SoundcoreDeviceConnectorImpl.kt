package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.content.Context
import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.lib.bindings.ConnectionWriter
import com.oppzippy.openscq30.lib.bindings.MacAddr6
import com.oppzippy.openscq30.lib.bindings.ManualConnection
import com.oppzippy.openscq30.lib.bindings.Uuid
import com.oppzippy.openscq30.lib.bindings.newSoundcoreDevice
import com.oppzippy.openscq30.lib.bindings.rfcommSppUuid
import java.util.concurrent.TimeoutException
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.launch
import kotlinx.coroutines.withTimeout

class SoundcoreDeviceConnectorImpl(
    private val context: Context,
    private val deviceFinder: BluetoothDeviceFinder,
    private val createManualConnection: (
        name: String,
        macAddress: MacAddr6,
        serviceUuid: Uuid,
        connectionWriter: ConnectionWriter,
    ) -> ManualConnection,
) : SoundcoreDeviceConnector {
    @Throws(SecurityException::class)
    override suspend fun connectToSoundcoreDevice(
        macAddress: String,
        coroutineScope: CoroutineScope,
    ): SoundcoreDevice? {
        var gatt: BluetoothGatt? = null
        var connection: ManualConnection? = null
        try {
            val bluetoothDevice = deviceFinder.findByMacAddress(macAddress) ?: return null
            val socket = bluetoothDevice.createRfcommSocketToServiceRecord(rfcommSppUuid())
            socket.connect()

            val callbacks =
                SoundcoreDeviceCallbackHandler(context = context, socket = socket)
            gatt = bluetoothDevice.connectGatt(context, false, callbacks, BluetoothDevice.TRANSPORT_LE)

            if (gatt.discoverServices()) {
                Log.d(
                    "SoundcoreDeviceConnectorImpl",
                    "Started discovering services, so we must be connected already",
                )
            } else {
                Log.d(
                    "SoundcoreDeviceConnectorImpl",
                    "Failed to start discovering services, so we must not be connected yet. Discovery should start upon connection.",
                )
            }

            try {
                withTimeout(4.seconds) {
                    callbacks.waitUntilReady()
                }
            } catch (ex: TimeoutCancellationException) {
                throw TimeoutException("Timeout waiting for GATT services").initCause(ex)
            }

            val serviceUuid = callbacks.serviceUuid.value ?: Uuid(0, 0)
            connection = createManualConnection(
                bluetoothDevice.name,
                bluetoothDevice.address,
                serviceUuid,
                callbacks,
            )
            callbacks.setManualConnection(connection)

            val nativeDevice = newSoundcoreDevice(connection)

            val soundcoreDevice = SoundcoreDevice(
                name = nativeDevice.name(),
                macAddress = nativeDevice.macAddress(),
                bleServiceUuid = nativeDevice.serviceUuid(),
                cleanUp = {
                    socket.close()
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
        } catch (ex: Exception) {
            Log.d("SoundcoreDeviceConnectorImpl", "Exception thrown, cleaning up resources")
            gatt?.disconnect()
            gatt?.close()
            connection?.close()
            throw ex
        }
    }
}

class GattServiceNotFoundException : RuntimeException()
