package com.oppzippy.openscq30.features.autoconnect

import android.bluetooth.BluetoothDevice
import android.companion.CompanionDeviceManager
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import androidx.core.content.IntentCompat
import com.oppzippy.openscq30.features.preferences.Preferences
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService

class BluetoothConnectionReceiver(private val preferences: Preferences) : BroadcastReceiver() {
    override fun onReceive(context: Context?, intent: Intent?) {
        Log.d("BluetoothConnectionReceiver", "got onReceive")
        if (context != null &&
            intent != null &&
            intent.action == BluetoothDevice.ACTION_ACL_CONNECTED &&
            preferences.autoConnect
        ) {
            val device = IntentCompat.getParcelableExtra(
                intent,
                BluetoothDevice.EXTRA_DEVICE,
                BluetoothDevice::class.java,
            )
            if (device != null) {
                val deviceManager = context.getSystemService(CompanionDeviceManager::class.java)
                val isPaired =
                    deviceManager.associations.find {
                        it.equals(device.address, ignoreCase = true)
                    } != null
                if (isPaired) {
                    Log.d("BluetoothConnectionReceiver", "auto connecting to ${device.address}")
                    val serviceIntent = Intent(context, DeviceService::class.java)
                    serviceIntent.putExtra(DeviceService.MAC_ADDRESS, device.address)
                    context.startForegroundService(serviceIntent)
                }
            }
        }
    }
}
