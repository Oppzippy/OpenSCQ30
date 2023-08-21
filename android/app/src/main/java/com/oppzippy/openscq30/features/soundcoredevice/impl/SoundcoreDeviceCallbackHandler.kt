package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.annotation.SuppressLint
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCallback
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattDescriptor
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothProfile
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.util.Log
import com.oppzippy.openscq30.lib.bindings.RequestFirmwareVersionPacket
import com.oppzippy.openscq30.lib.bindings.RequestStatePacket
import com.oppzippy.openscq30.lib.bindings.SoundcoreDeviceUtils
import kotlinx.coroutines.channels.BufferOverflow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import java.util.UUID
import java.util.concurrent.ConcurrentLinkedQueue

@SuppressLint("MissingPermission")
@Suppress("DEPRECATION")
class SoundcoreDeviceCallbackHandler(context: Context) : BluetoothGattCallback() {
    private lateinit var gatt: BluetoothGatt
    private var readCharacteristic: BluetoothGattCharacteristic? = null
    private var writeCharacteristic: BluetoothGattCharacteristic? = null
    private var commandQueue: ConcurrentLinkedQueue<Command> = ConcurrentLinkedQueue()
    private var isLocked: Boolean = false
    private val _packetsFlow: MutableSharedFlow<Packet> =
        MutableSharedFlow(0, 50, BufferOverflow.DROP_OLDEST)
    val packetsFlow = _packetsFlow.asSharedFlow()
    private val _isDisconnected = MutableStateFlow(false)
    val isDisconnected = _isDisconnected.asStateFlow()

    val adapter: BluetoothAdapter = context.getSystemService(BluetoothManager::class.java).adapter
    val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            val action = intent?.action
            if (action == BluetoothAdapter.ACTION_STATE_CHANGED) {
                if (adapter.state == BluetoothAdapter.STATE_TURNING_OFF || adapter.state == BluetoothAdapter.STATE_OFF) {
                    _isDisconnected.value = true
                }
            }
        }
    }

    init {
        context.registerReceiver(
            broadcastReceiver,
            IntentFilter(BluetoothAdapter.ACTION_STATE_CHANGED),
        )
    }

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
                    val isSuccess = when (command) {
                        is Command.SetMtu -> gatt.requestMtu(command.mtu)

                        Command.Read -> gatt.readCharacteristic(readCharacteristic)

                        is Command.Write -> {
                            writeCharacteristic.value = command.bytes
                            gatt.writeCharacteristic(writeCharacteristic)
                        }

                        is Command.WriteDescriptor -> {
                            command.descriptor.value = command.value
                            gatt.writeDescriptor(command.descriptor)
                        }
                    }
                    isLocked = isSuccess
                    if (!isSuccess) {
                        Log.w("SoundcoreDeviceCallbacks", "Command failed: $command")
                        _isDisconnected.value = true
                    }
                }
            }
        }
    }

    @Synchronized
    override fun onCharacteristicWrite(
        _gatt: BluetoothGatt?,
        characteristic: BluetoothGattCharacteristic?,
        status: Int,
    ) {
        isLocked = false
        next()
    }

    @Synchronized
    override fun onConnectionStateChange(
        gatt: BluetoothGatt?,
        status: Int,
        newState: Int,
    ) {
        if (newState == BluetoothProfile.STATE_CONNECTED && gatt != null) {
            this.gatt = gatt
            gatt.discoverServices()
        }
        if (newState == BluetoothProfile.STATE_DISCONNECTED || newState == BluetoothProfile.STATE_DISCONNECTING) {
            _isDisconnected.value = true
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
        _gatt: BluetoothGatt?,
        characteristic: BluetoothGattCharacteristic?,
    ) {
        if (characteristic != null) {
            val bytes = characteristic.value
            val packet = Packet.fromBytes(bytes)
            if (packet != null) {
                if (!_packetsFlow.tryEmit(packet)) {
                    Log.e("SoundcoreDeviceCallbacks", "failed to emit packet to flow")
                }
            } else {
                Log.i("unknown-packet", "got unknown packet: $bytes")
            }
            isLocked = false
            next()
        }
    }

    @Synchronized
    override fun onDescriptorWrite(
        _gatt: BluetoothGatt?,
        descriptor: BluetoothGattDescriptor?,
        status: Int,
    ) {
        isLocked = false
        next()
    }

    @Synchronized
    override fun onDescriptorRead(
        _gatt: BluetoothGatt?,
        descriptor: BluetoothGattDescriptor?,
        status: Int,
    ) {
        isLocked = false
        next()
    }

    @Synchronized
    override fun onServicesDiscovered(_gatt: BluetoothGatt?, status: Int) {
        val service = gatt.services.first {
            SoundcoreDeviceUtils.isSoundcoreServiceUuid(
                it.uuid.mostSignificantBits,
                it.uuid.leastSignificantBits,
            )
        }

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
                descriptor,
                BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE,
            ),
        )
        queueCommanad(Command.SetMtu(500))
        queueCommanad(Command.Write(RequestStatePacket().bytes()))
        // TODO only send this if the preceding state update packet did not get this information
        queueCommanad(Command.Write(RequestFirmwareVersionPacket().bytes()))
    }
}
