package com.oppzippy.openscq30.features.soundcoredevice

import android.Manifest
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothSocket
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.app.ActivityCompat
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.AndroidException
import com.oppzippy.openscq30.lib.bindings.AndroidRfcommConnectionBackend
import com.oppzippy.openscq30.lib.bindings.AndroidRfcommConnectionWriter
import com.oppzippy.openscq30.lib.bindings.MacAddr6
import com.oppzippy.openscq30.lib.bindings.ManualConnectionBackends
import com.oppzippy.openscq30.lib.bindings.ManualRfcommConnection
import com.oppzippy.openscq30.lib.bindings.ManualRfcommConnectionBox
import com.oppzippy.openscq30.lib.bindings.RfcommServiceSelectionStrategy
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import com.oppzippy.openscq30.lib.wrapper.ConnectionStatus
import java.io.IOException
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

fun connectionBackends(context: Context, coroutineScope: CoroutineScope): ManualConnectionBackends =
    ManualConnectionBackends(
        rfcomm = AndroidRfcommConnectionBackendImpl(context, coroutineScope),
    )

class AndroidRfcommConnectionBackendImpl(private val context: Context, private val coroutineScope: CoroutineScope) :
    AndroidRfcommConnectionBackend {
    companion object {
        private const val TAG = "AndroidRfcommConnectionBackendImpl"
    }

    override suspend fun devices(): List<ConnectionDescriptor> {
        try {
            val bluetoothManager: BluetoothManager? = context.getSystemService(BluetoothManager::class.java)
            if (bluetoothManager == null) {
                Log.e(TAG, "BluetoothManager is null. Does the system not support bluetooth?")
                return emptyList()
            }

            if (
                ActivityCompat.checkSelfPermission(
                    context,
                    Manifest.permission.BLUETOOTH_CONNECT,
                ) != PackageManager.PERMISSION_GRANTED
            ) {
                Log.e(TAG, "Missing BLUETOOTH_CONNECT permission")
                return emptyList()
            }

            val bondedDevices: Set<BluetoothDevice>? = bluetoothManager.adapter.bondedDevices
            if (bondedDevices == null) {
                Log.e(TAG, "bondedDevices is null, see preceding error message from bluetooth adapter")
                return emptyList()
            }

            return bondedDevices.map {
                val name: String? = it.name
                if (name == null) {
                    Log.w(TAG, "bonded device with mac address ${it.address} has null name")
                }
                ConnectionDescriptor(
                    name = name ?: context.getString(R.string.unknown),
                    macAddress = it.address,
                )
            }
        } catch (ex: CancellationException) {
            throw ex
        } catch (ex: Exception) {
            throw AndroidException.Other(ex.stackTraceToString())
        }
    }

    override suspend fun connect(
        macAddress: MacAddr6,
        serviceSelectionStrategy: RfcommServiceSelectionStrategy,
        outputBox: ManualRfcommConnectionBox,
    ) {
        if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.BLUETOOTH_CONNECT,
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            Log.e(TAG, "Missing BLUETOOTH_CONNECT permission")
            return
        }

        val bluetoothManager: BluetoothManager? = context.getSystemService(BluetoothManager::class.java)
        if (bluetoothManager == null) {
            Log.e(TAG, "BluetoothManager is null. Does the system not support bluetooth?")
            return
        }

        val bondedDevices: Set<BluetoothDevice>? = bluetoothManager.adapter.bondedDevices
        if (bondedDevices == null) {
            Log.e(TAG, "bondedDevices is null, see preceding error message from bluetooth adapter")
            return
        }
        val device = bondedDevices.find { it.address == macAddress }
        if (device == null) {
            Log.w(TAG, "device with mac address $macAddress not found")
            return
        }

        Log.d(TAG, "found device $macAddress")
        val uuid = when (serviceSelectionStrategy) {
            is RfcommServiceSelectionStrategy.Constant -> serviceSelectionStrategy.uuid

            is RfcommServiceSelectionStrategy.Dynamic -> {
                val uuids = device.uuids?.map { it.uuid }
                if (uuids == null) {
                    Log.e(TAG, "error getting device service uuids")
                    return
                }
                Log.d(TAG, "found uuids: $uuids")
                serviceSelectionStrategy.selectService.select(uuids)
            }
        }

        Log.d(TAG, "selected uuid $uuid")

        val socket = try {
            device.createRfcommSocketToServiceRecord(uuid)
        } catch (ex: IOException) {
            Log.e(TAG, "error creating rfcomm socket", ex)
            return
        }

        try {
            withContext(Dispatchers.IO) {
                socket.connect()
            }
        } catch (_: CancellationException) {
            try {
                Log.d(TAG, "connection canceled, closing socket")
                socket.close()
                return
            } catch (ex: IOException) {
                Log.d(TAG, "error closing socket during cancellation", ex)
            }
        } catch (ex: IOException) {
            Log.w(TAG, "error connecting to device", ex)
            throw AndroidException.Other("error connecting to device")
        }

        var manualRfcommConnection: ManualRfcommConnection? = null
        manualRfcommConnection = ManualRfcommConnection(
            AndroidRfcommConnectionWriterImpl(
                socket = socket,
                setConnectionStatus = { manualRfcommConnection?.setConnectionStatus(it) },
            ),
        )
        coroutineScope.launch {
            withContext(Dispatchers.IO) {
                while (true) {
                    try {
                        val buffer = ByteArray(1000)
                        // The socket will be closed from the rust side when we disconnect from the device, so when that
                        // happens, this will throw, and we'll break out of the loop
                        when (val size = socket.inputStream.read(buffer)) {
                            -1 -> {
                                Log.d(TAG, "end of stream")
                                break
                            }

                            0 -> Unit

                            else -> manualRfcommConnection.addInboundPacket(buffer.sliceArray(0..<size))
                        }
                    } catch (ex: IOException) {
                        Log.d(TAG, "disconnected", ex)
                        break
                    }
                }
                manualRfcommConnection.setConnectionStatus(ConnectionStatus.Disconnected)
                try {
                    socket.close()
                } catch (ex: IOException) {
                    Log.d(TAG, "closing socket", ex)
                }
            }
        }

        outputBox.set(manualRfcommConnection)
    }
}

class AndroidRfcommConnectionWriterImpl(
    private val socket: BluetoothSocket,
    private val setConnectionStatus: (ConnectionStatus) -> Unit,
) : AndroidRfcommConnectionWriter {
    companion object {
        private const val TAG = "AndroidRfcommConnectionWriterImpl"
    }

    override suspend fun write(data: ByteArray) {
        withContext(Dispatchers.IO) {
            try {
                socket.outputStream.write(data)
            } catch (ex: IOException) {
                Log.d(TAG, "disconnected", ex)
                setConnectionStatus(ConnectionStatus.Disconnected)
            }
        }
    }

    override fun closeSocket() {
        try {
            socket.close()
        } catch (ex: IOException) {
            Log.d(TAG, "closing socket", ex)
        }
    }
}
