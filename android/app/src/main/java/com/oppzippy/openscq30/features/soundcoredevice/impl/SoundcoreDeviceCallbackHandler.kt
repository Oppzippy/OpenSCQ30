package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.annotation.SuppressLint
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCallback
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothProfile
import android.bluetooth.BluetoothSocket
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.util.Log
import com.oppzippy.openscq30.lib.bindings.ConnectionWriter
import com.oppzippy.openscq30.lib.bindings.ManualConnection
import com.oppzippy.openscq30.lib.bindings.isSoundcoreServiceUuid
import java.io.IOException
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

@SuppressLint("MissingPermission")
class SoundcoreDeviceCallbackHandler(context: Context, private val socket: BluetoothSocket) :
    BluetoothGattCallback(),
    ConnectionWriter,
    AutoCloseable {
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
                if (adapter.state == BluetoothAdapter.STATE_TURNING_OFF ||
                    adapter.state == BluetoothAdapter.STATE_OFF
                ) {
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

        Thread {
            try {
                while (true) {
                    val buffer = ByteArray(1024)
                    when (val bytesRead = socket.inputStream.read(buffer)) {
                        -1 -> {
                            Log.i("SoundcoreDeviceCallbackHandler", "read returned -1 (end of stream), disconnecting")
                            break
                        }

                        0 -> Unit
                        else -> manualConnection?.addInboundPacket(buffer.copyOf(bytesRead))
                    }
                }
            } catch (ex: IOException) {
                Log.i("SoundcoreDeviceCallbackHandler", "read failed, disconnecting", ex)
            }
            _isDisconnected.value = true
        }.start()
    }

    @Synchronized
    override fun close() {
        manualConnection?.close()
        socket.close()
    }

    override fun writeWithResponse(data: ByteArray) {
        writeWithoutResponse(data)
    }

    override fun writeWithoutResponse(data: ByteArray) {
        socket.outputStream.write(data)
    }

    private val ready: Mutex = Mutex(locked = true)

    suspend fun waitUntilReady() {
        // A bit jank since we don't need the lock, we just need to wait until it is not locked
        // A ConditionVariable would probably be better
        ready.withLock { }
    }

    @Synchronized
    override fun onConnectionStateChange(gatt: BluetoothGatt?, status: Int, newState: Int) {
        if (newState == BluetoothProfile.STATE_CONNECTED && gatt != null) {
            gatt.discoverServices()
        }
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
        if (status != BluetoothGatt.GATT_SUCCESS) {
            Log.e(
                "SoundcoreDeviceCallbackHandler",
                "gatt service discovery failed with status $status",
            )
            return
        }
        gatt.services.firstOrNull {
            isSoundcoreServiceUuid(it.uuid)
        }?.let { service ->
            _serviceUuid.value = service.uuid
        }

        try {
            ready.unlock()
        } catch (ex: IllegalStateException) {
            // ready will only be locked once, so we only care about unlocking it once.
        }
    }
}
