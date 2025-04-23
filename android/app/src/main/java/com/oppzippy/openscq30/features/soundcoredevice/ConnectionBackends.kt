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
import com.oppzippy.openscq30.lib.bindings.RfcommConnectionWriterBox
import com.oppzippy.openscq30.lib.bindings.UuidSelector
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

fun connectionBackends(context: Context): ManualConnectionBackends {
    return ManualConnectionBackends(
        rfcomm = AndroidRfcommConnectionBackendImpl(context),
    )
}

class AndroidRfcommConnectionBackendImpl(private val context: Context) : AndroidRfcommConnectionBackend {
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

    override suspend fun connect(macAddress: MacAddr6, selectUuid: UuidSelector, outputBox: RfcommConnectionWriterBox) {
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
        outputBox.set(AndroidRfcommConnectionWriterImpl(socket))
    }
}

class AndroidRfcommConnectionWriterImpl(private val socket: BluetoothSocket) : AndroidRfcommConnectionWriter {
    override suspend fun write(data: ByteArray) {
        withContext(Dispatchers.IO) {
            socket.outputStream.write(data)
        }
    }
}

