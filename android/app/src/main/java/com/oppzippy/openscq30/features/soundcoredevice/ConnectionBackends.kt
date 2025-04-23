package com.oppzippy.openscq30.features.soundcoredevice

import android.Manifest
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothSocket
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.app.ActivityCompat
import com.oppzippy.openscq30.lib.bindings.AndroidRfcommConnectionBackend
import com.oppzippy.openscq30.lib.bindings.AndroidRfcommConnectionWriter
import com.oppzippy.openscq30.lib.bindings.MacAddr6
import com.oppzippy.openscq30.lib.bindings.ManualConnectionBackends
import com.oppzippy.openscq30.lib.bindings.ManualRfcommConnection
import com.oppzippy.openscq30.lib.bindings.ManualRfcommConnectionBox
import com.oppzippy.openscq30.lib.bindings.UuidSelector
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import com.oppzippy.openscq30.lib.wrapper.ConnectionStatus
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.IOException

fun connectionBackends(context: Context, coroutineScope: CoroutineScope): ManualConnectionBackends {
    return ManualConnectionBackends(
        rfcomm = AndroidRfcommConnectionBackendImpl(context, coroutineScope),
    )
}

class AndroidRfcommConnectionBackendImpl(private val context: Context, private val coroutineScope: CoroutineScope) :
    AndroidRfcommConnectionBackend {
    override suspend fun devices(): List<ConnectionDescriptor> {
        val bluetoothManager: BluetoothManager = context.getSystemService(BluetoothManager::class.java)
        return if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.BLUETOOTH_CONNECT,
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            Log.e("AndroidRfcommConnectionBackendImpl", "Missing BLUETOOTH_CONNECT permission")
            emptyList()
        } else {
            bluetoothManager.adapter.bondedDevices.map {
                ConnectionDescriptor(name = it.name, macAddress = it.address)
            }
        }
    }

    override suspend fun connect(macAddress: MacAddr6, selectUuid: UuidSelector, outputBox: ManualRfcommConnectionBox) {
        if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.BLUETOOTH_CONNECT,
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            Log.e("AndroidRfcommConnectionBackendImpl", "Missing BLUETOOTH_CONNECT permission")
            return
        }
        val bluetoothManager: BluetoothManager = context.getSystemService(BluetoothManager::class.java)
        val device = bluetoothManager.adapter.bondedDevices.find { it.address == macAddress } ?: return
        val uuid = selectUuid.select(device.uuids.map { it.uuid })
        val socket = device.createRfcommSocketToServiceRecord(uuid)
        try {
            withContext(Dispatchers.IO) {
                socket.connect()
            }
        } catch (ex: CancellationException) {
            socket.close()
        }

        var manualRfcommConnection: ManualRfcommConnection? = null
        manualRfcommConnection =
            ManualRfcommConnection(
                AndroidRfcommConnectionWriterImpl(
                    socket,
                    { manualRfcommConnection?.setConnectionStatus(it) },
                ),
            )
        coroutineScope.launch {
            withContext(Dispatchers.IO) {
                while (true) {
                    try {
                        val inboundPacket = socket.inputStream.readBytes()
                        manualRfcommConnection.addInboundPacket(inboundPacket)
                    } catch (ex: IOException) {
                        Log.d("AndroidRfcommConnectionBackendImpl", "disconnected", ex)
                        break
                    }
                }
                manualRfcommConnection.setConnectionStatus(ConnectionStatus.Disconnected)
                socket.close()
            }
        }

        outputBox.set(manualRfcommConnection)
    }
}

class AndroidRfcommConnectionWriterImpl(
    private val socket: BluetoothSocket,
    private val setConnectionStatus: (ConnectionStatus) -> Unit,
) : AndroidRfcommConnectionWriter {
    override suspend fun write(data: ByteArray) {
        withContext(Dispatchers.IO) {
            try {
                socket.outputStream.write(data)
            } catch (ex: IOException) {
                Log.d("AndroidRfcommConnectionWriterImpl", "disconnected", ex)
                setConnectionStatus(ConnectionStatus.Disconnected)
            }
        }
    }
}

