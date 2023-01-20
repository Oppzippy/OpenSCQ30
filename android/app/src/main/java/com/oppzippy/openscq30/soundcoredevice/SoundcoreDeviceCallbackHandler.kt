package com.oppzippy.openscq30.soundcoredevice

import android.annotation.SuppressLint
import android.annotation.TargetApi
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCallback
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattDescriptor
import android.bluetooth.BluetoothProfile
import android.util.Log
import com.oppzippy.openscq30.lib.AmbientSoundModeUpdatePacket
import com.oppzippy.openscq30.lib.OkPacket
import com.oppzippy.openscq30.lib.RequestStatePacket
import com.oppzippy.openscq30.lib.SoundcoreDeviceUtils
import com.oppzippy.openscq30.lib.StateUpdatePacket
import kotlinx.coroutines.channels.BufferOverflow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import java.util.*
import java.util.concurrent.ConcurrentLinkedQueue
import kotlin.jvm.optionals.getOrNull

@SuppressLint("MissingPermission")
@Suppress("DEPRECATION")
class SoundcoreDeviceCallbackHandler() : BluetoothGattCallback() {
    private lateinit var gatt: BluetoothGatt
    private var readCharacteristic: BluetoothGattCharacteristic? = null
    private var writeCharacteristic: BluetoothGattCharacteristic? = null
    private var commandQueue: ConcurrentLinkedQueue<Command> = ConcurrentLinkedQueue()
    private var isLocked: Boolean = false
    private val _packetsFlow: MutableSharedFlow<Packet> =
        MutableSharedFlow(0, 50, BufferOverflow.DROP_OLDEST)
    val packetsFlow: SharedFlow<Packet> = _packetsFlow

    @Synchronized
    fun queueCommanad(command: Command) {
        commandQueue.add(command)
        next()
    }

    @Synchronized
    private fun next() {
        if (!isLocked) {
            val writeCharacteristic = writeCharacteristic
            val readCharacteristic = readCharacteristic
            if (writeCharacteristic != null && readCharacteristic != null) {
                commandQueue.poll()?.let { command ->
                    when (command) {
                        Command.Read -> {
                            if (!gatt.readCharacteristic(readCharacteristic)) {
                                Log.w("SoundcoreDeviceCallbaks", "readCharacteristic failed")
                            }
                        }
                        is Command.Write -> {
                            writeCharacteristic.value = command.bytes
                            if (!gatt.writeCharacteristic(writeCharacteristic)) {
                                Log.w("SoundcoreDeviceCallbaks", "writeCharacteristic failed")
                            }
                        }
                        is Command.SetMtu -> {
                            if (!gatt.requestMtu(command.mtu)) {
                                Log.w("SoundcoreDeviceCallbaks", "requestMtu failed")
                            }
                        }
                        is Command.WriteDescriptor -> {
                            command.descriptor.value = command.value
                            if (!gatt.writeDescriptor(command.descriptor)) {
                                Log.w("SoundcoreDeviceCallbaks", "writeDescriptor failed")
                            }
                        }
                    }
                    isLocked = true
                }
            }
        }
    }

    @Synchronized
    override fun onCharacteristicWrite(
        _gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?, status: Int
    ) {
        isLocked = false
        next()
    }


    @Synchronized
    override fun onConnectionStateChange(
        gatt: BluetoothGatt?, status: Int, newState: Int
    ) {
        if (newState == BluetoothProfile.STATE_CONNECTED && gatt != null) {
            this.gatt = gatt
            gatt.discoverServices()
        }
    }

    @Synchronized
    override fun onMtuChanged(_gatt: BluetoothGatt?, mtu: Int, status: Int) {
        Log.i("SoundcoreDeviceCallbaks", "mtu changed to $mtu, status $status")
        isLocked = false
        next()
    }

    @Synchronized
    override fun onCharacteristicChanged(
        _gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?
    ) {
        if (characteristic != null) {
            val value = characteristic.value
            AmbientSoundModeUpdatePacket.fromBytes(value).getOrNull()?.let {
                _packetsFlow.tryEmit(Packet.AmbientSoundModeUpdate(it))
                return
            }
            StateUpdatePacket.fromBytes(value).getOrNull()?.let {
                _packetsFlow.tryEmit(Packet.StateUpdate(it))
                return
            }
            OkPacket.fromBytes(value).getOrNull()?.let {
                _packetsFlow.tryEmit(Packet.Ok(it))
                return
            }
            Log.i("unknown-packet", "got unknown packet")
            isLocked = false
            next()
        }
    }

    @Synchronized
    override fun onDescriptorWrite(
        _gatt: BluetoothGatt?, descriptor: BluetoothGattDescriptor?, status: Int
    ) {
        isLocked = false
        next()
    }

    @Synchronized
    override fun onDescriptorRead(
        _gatt: BluetoothGatt?, descriptor: BluetoothGattDescriptor?, status: Int
    ) {
        isLocked = false
        next()
    }

    @Synchronized
    override fun onServicesDiscovered(_gatt: BluetoothGatt?, status: Int) {
        val service = gatt.getService(UUID.fromString(SoundcoreDeviceUtils.serviceUuid()))
        writeCharacteristic =
            service.getCharacteristic(UUID.fromString(SoundcoreDeviceUtils.writeCharacteristicUuid()))
        val readCharacteristic =
            service.getCharacteristic(UUID.fromString(SoundcoreDeviceUtils.readCharacteristicUuid()))
        this.readCharacteristic = readCharacteristic

        gatt.setCharacteristicNotification(readCharacteristic, true)
        val descriptor =
            readCharacteristic.getDescriptor(UUID(0x0000290200001000, -9223371485494954757))

        queueCommanad(
            Command.WriteDescriptor(
                descriptor, BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE
            )
        )
        queueCommanad(Command.SetMtu(500))
        queueCommanad(Command.Write(RequestStatePacket().bytes()))
    }
}
