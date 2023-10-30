package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.bluetooth.BluetoothDevice
import android.content.Context
import android.util.Log
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.lib.bindings.RequestFirmwareVersionPacket
import com.oppzippy.openscq30.lib.bindings.RequestStatePacket
import com.oppzippy.openscq30.lib.wrapper.toSoundcoreDeviceState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.withTimeout
import kotlinx.coroutines.withTimeoutOrNull
import java.util.concurrent.TimeoutException
import kotlin.time.Duration.Companion.seconds

class SoundcoreDeviceConnectorImpl(
    private val context: Context,
    private val deviceFinder: BluetoothDeviceFinder,
    private val callbackHandlerFactory: SoundcoreDeviceCallbackHandlerFactory,
    private val deviceFactory: SoundcoreDeviceFactory,
) : SoundcoreDeviceConnector {
    @Throws(SecurityException::class)
    override suspend fun connectToSoundcoreDevice(
        macAddress: String,
        scope: CoroutineScope,
    ): SoundcoreDevice? {
        val bluetoothDevice = deviceFinder.findByMacAddress(macAddress) ?: return null
        val callbacks = callbackHandlerFactory.createSoundcoreDeviceCallbackHandler(context)
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

        var packet: Packet.StateUpdate? = null
        for (i in 0..3) {
            callbacks.queueCommand(Command.Write(RequestStatePacket().bytes()))
            packet = withTimeoutOrNull(1.seconds) {
                callbacks.packetsFlow.first { it is Packet.StateUpdate } as Packet.StateUpdate
            }
            if (packet != null) {
                break
            }
        }

        if (packet == null) {
            throw TimeoutException("Timeout waiting for initial device state")
        }

        callbacks.queueCommand(Command.Write(RequestFirmwareVersionPacket().bytes()))

        return deviceFactory.createSoundcoreDevice(
            gatt,
            callbacks,
            scope,
            packet.inner.toSoundcoreDeviceState(),
        )
    }
}
