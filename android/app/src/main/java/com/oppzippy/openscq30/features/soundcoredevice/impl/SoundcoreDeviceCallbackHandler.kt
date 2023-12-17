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
import com.oppzippy.openscq30.lib.bindings.ConnectionWriter
import com.oppzippy.openscq30.lib.bindings.ManualConnection
import com.oppzippy.openscq30.lib.bindings.isSoundcoreServiceUuid
import com.oppzippy.openscq30.lib.bindings.readCharacteristicUuid
import com.oppzippy.openscq30.lib.bindings.writeCharacteristicUuid
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import java.util.UUID
import java.util.concurrent.ConcurrentLinkedQueue

@SuppressLint("MissingPermission")
@Suppress("DEPRECATION")
class SoundcoreDeviceCallbackHandler(
    context: Context,
    private val coroutineScope: CoroutineScope,
) :
    BluetoothGattCallback(), ConnectionWriter, AutoCloseable {
    private var gatt: BluetoothGatt? = null
    private var readCharacteristic: BluetoothGattCharacteristic? = null
    private var writeCharacteristic: BluetoothGattCharacteristic? = null
    private var commandQueue: ConcurrentLinkedQueue<Command> = ConcurrentLinkedQueue()
    private var isLocked: Boolean = false
    private val _isDisconnected = MutableStateFlow(false)
    val isDisconnected = _isDisconnected.asStateFlow()
    private var _serviceUuid = MutableStateFlow<UUID?>(null)
    val serviceUuid = _serviceUuid.asStateFlow()
    private var manualConnection: ManualConnection? = null

    @Synchronized
    fun setManualConnection(manualConnection: ManualConnection) {
        this.manualConnection?.close()
        this.manualConnection = manualConnection
    }

    val adapter: BluetoothAdapter = context.getSystemService(BluetoothManager::class.java).adapter
    private val broadcastReceiver = object : BroadcastReceiver() {
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
    override fun close() {
        manualConnection?.close()
    }

    override fun writeWithResponse(data: ByteArray) {
        queueCommand(Command.Write(data))
    }

    override fun writeWithoutResponse(data: ByteArray) {
        queueCommand(Command.Write(data))
    }

    @Synchronized
    fun queueCommand(command: Command) {
        commandQueue.add(command)
        next()
    }

    @Synchronized
    private fun next() {
        if (!isLocked) {
            val writeCharacteristic = writeCharacteristic
            val readCharacteristic = readCharacteristic
            val gatt = gatt
            if (gatt != null && writeCharacteristic != null && readCharacteristic != null) {
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
        gatt: BluetoothGatt?,
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
    override fun onMtuChanged(gatt: BluetoothGatt?, mtu: Int, status: Int) {
        Log.i("SoundcoreDeviceCallbaks", "mtu changed to $mtu, status $status")
        isLocked = false
        next()
    }

    @Deprecated("Deprecated in Java")
    @Synchronized
    override fun onCharacteristicChanged(
        gatt: BluetoothGatt?,
        characteristic: BluetoothGattCharacteristic?,
    ) {
        if (characteristic != null) {
            val bytes = characteristic.value
            coroutineScope.launch {
                manualConnection?.addInboundPacket(bytes)
            }
            isLocked = false
            next()
        }
    }

    @Synchronized
    override fun onDescriptorWrite(
        gatt: BluetoothGatt?,
        descriptor: BluetoothGattDescriptor?,
        status: Int,
    ) {
        isLocked = false
        next()
    }

    @Deprecated("Deprecated in Java")
    @Synchronized
    override fun onDescriptorRead(
        gatt: BluetoothGatt?,
        descriptor: BluetoothGattDescriptor?,
        status: Int,
    ) {
        isLocked = false
        next()
    }

    private val ready: Mutex = Mutex(locked = true)

    suspend fun waitUntilReady() {
        // A bit jank since we don't need the lock, we just need to wait until it is not locked
        // A ConditionVariable would probably be better
        ready.withLock { }
    }

    @Synchronized
    override fun onServicesDiscovered(gatt: BluetoothGatt?, status: Int) {
        if (gatt == null) {
            Log.e(
                "SoundcoreDeviceCallbackHandler",
                "onServicesDiscovered: gatt is null? status $status",
            )
            return
        }
        val service = gatt.services.first {
            isSoundcoreServiceUuid(it.uuid)
        }
        _serviceUuid.value = service.uuid

        writeCharacteristic =
            service.getCharacteristic(UUID.fromString(writeCharacteristicUuid()))
        val readCharacteristic =
            service.getCharacteristic(UUID.fromString(readCharacteristicUuid()))
        this.readCharacteristic = readCharacteristic

        gatt.setCharacteristicNotification(readCharacteristic, true)
        val descriptor =
            readCharacteristic.getDescriptor(UUID(0x0000290200001000, -9223371485494954757))

        queueCommand(
            Command.WriteDescriptor(
                descriptor,
                BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE,
            ),
        )
        queueCommand(Command.SetMtu(500))
        ready.unlock()
    }
}
