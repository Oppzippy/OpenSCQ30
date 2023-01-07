package com.oppzippy.openscq30.soundcoredevice

import android.Manifest
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCallback
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattService
import android.bluetooth.BluetoothProfile
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.app.ActivityCompat
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.lib.SetAmbientSoundModePacket
import java.util.*

class SoundcoreDevice(private val context: Context, private val bluetoothDevice: BluetoothDevice) {
    private lateinit var gatt: BluetoothGatt
    private var characteristic: BluetoothGattCharacteristic? = null
    init {
        if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.BLUETOOTH_CONNECT
            ) == PackageManager.PERMISSION_GRANTED
        ) {
            // TODO: Consider calling
            //    ActivityCompat#requestPermissions
            // here to request the missing permissions, and then overriding
            //   public void onRequestPermissionsResult(int requestCode, String[] permissions,
            //                                          int[] grantResults)
            // to handle the case where the user grants the permission. See the documentation
            // for ActivityCompat#requestPermissions for more details.
            gatt = bluetoothDevice.connectGatt(context, true, object : BluetoothGattCallback() {
                override fun onConnectionStateChange(
                    gatt: BluetoothGatt?,
                    status: Int,
                    newState: Int
                ) {
                    Log.i("state", (gatt != null).toString())
                    Log.i("state", newState.toString())
                    if (newState == BluetoothProfile.STATE_CONNECTED) {
                        if (ActivityCompat.checkSelfPermission(
                                context,
                                Manifest.permission.BLUETOOTH_CONNECT
                            ) != PackageManager.PERMISSION_GRANTED
                        ) {
                            // TODO: Consider calling
                            //    ActivityCompat#requestPermissions
                            // here to request the missing permissions, and then overriding
                            //   public void onRequestPermissionsResult(int requestCode, String[] permissions,
                            //                                          int[] grantResults)
                            // to handle the case where the user grants the permission. See the documentation
                            // for ActivityCompat#requestPermissions for more details.
                            return
                        }
                        gatt?.discoverServices()
                    }
                }
                override fun onServicesDiscovered(gatt: BluetoothGatt?, status: Int) {
                    Log.i("onServicesDiscovered", (gatt == null).toString())
                    if (gatt != null) {
                        val service = gatt.getService(UUID.fromString("011cf5da-0000-1000-8000-00805f9b34fb"))
                        Log.i("found-service?", (service != null).toString())
                        characteristic = service.getCharacteristic(UUID.fromString("00007777-0000-1000-8000-00805f9b34fb"))
                        Log.i("found-characteristic?", (characteristic != null).toString())
                    }
                    super.onServicesDiscovered(gatt, status)
                }
            }, BluetoothDevice.TRANSPORT_LE)
        }
        gatt.connect()
    }

   fun setAmbientSoundMode(mode: AmbientSoundMode) {
       val characteristic = characteristic
        if (characteristic != null) {
            characteristic.value = SetAmbientSoundModePacket(mode, NoiseCancelingMode.Transport).bytes().map { it.toByte() }.toByteArray()
            if (ActivityCompat.checkSelfPermission(
                    context,
                    Manifest.permission.BLUETOOTH_CONNECT
                ) != PackageManager.PERMISSION_GRANTED
            ) {
                // TODO: Consider calling
                //    ActivityCompat#requestPermissions
                // here to request the missing permissions, and then overriding
                //   public void onRequestPermissionsResult(int requestCode, String[] permissions,
                //                                          int[] grantResults)
                // to handle the case where the user grants the permission. See the documentation
                // for ActivityCompat#requestPermissions for more details.
                return
            }
            gatt.writeCharacteristic(characteristic)
        }
    }
}