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
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import com.oppzippy.openscq30.lib.StateUpdatePacket
import kotlinx.coroutines.flow.MutableStateFlow
import java.util.*
import java.util.concurrent.ConcurrentLinkedQueue
import kotlin.jvm.optionals.getOrNull

@SuppressLint("MissingPermission")
class SoundcoreDeviceCallbacks(private val mutableState: MutableStateFlow<SoundcoreDeviceState?>) :
    BluetoothGattCallback() {
    private var readCharacteristic: BluetoothGattCharacteristic? = null
    private var writeCharacteristic: BluetoothGattCharacteristic? = null
    private var commandQueue: ConcurrentLinkedQueue<Command> = ConcurrentLinkedQueue()
    private var isLocked: Boolean = false


    @Synchronized
    fun queueCommanad(gatt: BluetoothGatt, command: Command) {
        commandQueue.add(command)
        next(gatt)
    }

    @Synchronized
    private fun next(gatt: BluetoothGatt) {
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
        gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?, status: Int
    ) {
        isLocked = false
        if (gatt != null) {
            next(gatt)
        }
    }


    @Synchronized
    override fun onConnectionStateChange(
        gatt: BluetoothGatt?, status: Int, newState: Int
    ) {
        if (newState == BluetoothProfile.STATE_CONNECTED) {
            gatt?.discoverServices()
        }
    }

    @Synchronized
    override fun onCharacteristicRead(
        gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?, status: Int
    ) {
        if (gatt != null && characteristic != null) {
            val value = characteristic.value
            AmbientSoundModeUpdatePacket.fromBytes(value).getOrNull()?.let {
                mutableState.value = mutableState.value?.withAmbientSoundMode(it.ambientSoundMode())
                    ?.withNoiseCancelingMode(it.noiseCancelingMode())
                return
            }
            StateUpdatePacket.fromBytes(value).getOrNull()?.let {
                mutableState.value = SoundcoreDeviceState(it)
                return
            }
            OkPacket.fromBytes(value).getOrNull()?.let {
                return
            }
            Log.i("unknown-packet", "got unknown packet")
            isLocked = false
            next(gatt)
        }
    }

    @Synchronized
    override fun onMtuChanged(gatt: BluetoothGatt?, mtu: Int, status: Int) {
        Log.i("SoundcoreDeviceCallbaks", "mtu changed to $mtu, status $status")
        isLocked = false
        if (gatt != null) {
            next(gatt)
        }
    }

    @Synchronized
    override fun onCharacteristicChanged(
        gatt: BluetoothGatt?, characteristic: BluetoothGattCharacteristic?
    ) {
        if (gatt != null && characteristic != null) {
            val value = characteristic.value
            AmbientSoundModeUpdatePacket.fromBytes(value).getOrNull()?.let {
                mutableState.value = mutableState.value?.withAmbientSoundMode(it.ambientSoundMode())
                    ?.withNoiseCancelingMode(it.noiseCancelingMode())
                return
            }
            StateUpdatePacket.fromBytes(value).getOrNull()?.let {
                mutableState.value = SoundcoreDeviceState(it)
                return
            }
            OkPacket.fromBytes(value).getOrNull()?.let {
                return
            }
            Log.i("unknown-packet", "got unknown packet")
            isLocked = false
            next(gatt)
        }
    }

    @Synchronized
    override fun onDescriptorWrite(
        gatt: BluetoothGatt?, descriptor: BluetoothGattDescriptor?, status: Int
    ) {
        super.onDescriptorWrite(gatt, descriptor, status)
        isLocked = false
        if (gatt != null) {
            next(gatt)
        }
    }

    @Synchronized
    override fun onDescriptorRead(
        gatt: BluetoothGatt?, descriptor: BluetoothGattDescriptor?, status: Int
    ) {
        isLocked = false
        if (gatt != null) {
            next(gatt)
        }
    }

    @Synchronized
    override fun onServicesDiscovered(gatt: BluetoothGatt?, status: Int) {
        if (gatt != null) {
            val service = gatt.getService(UUID.fromString("011cf5da-0000-1000-8000-00805f9b34fb"))
            writeCharacteristic =
                service.getCharacteristic(UUID.fromString("00007777-0000-1000-8000-00805f9b34fb"))
            val readCharacteristic =
                service.getCharacteristic(UUID.fromString("00008888-0000-1000-8000-00805F9B34FB"))
            this.readCharacteristic = readCharacteristic

            gatt.setCharacteristicNotification(readCharacteristic, true)
            val descriptor =
                readCharacteristic.getDescriptor(UUID(0x0000290200001000, -9223371485494954757))

            queueCommanad(
                gatt, Command.WriteDescriptor(
                    descriptor, BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE
                )
            )
            queueCommanad(gatt, Command.SetMtu(500))
            queueCommanad(gatt, Command.Write(RequestStatePacket().bytes()))
        }
    }
}
