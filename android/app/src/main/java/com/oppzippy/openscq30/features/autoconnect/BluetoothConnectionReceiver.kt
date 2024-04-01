package com.oppzippy.openscq30.features.autoconnect

import android.bluetooth.BluetoothDevice
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import androidx.core.content.IntentCompat
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService

class BluetoothConnectionReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context?, intent: Intent?) {
        Log.d("BluetoothConnectionReceiver", "got onreceive")
        if (context != null && intent != null && intent.action == BluetoothDevice.ACTION_ACL_CONNECTED) {
            Log.d("BluetoothConnectionReceiver", "attempting to start background service")
            val device = IntentCompat.getParcelableExtra(
                intent,
                BluetoothDevice.EXTRA_DEVICE,
                BluetoothDevice::class.java,
            )
            if (device != null) {
                val serviceIntent = Intent(context, DeviceService::class.java)
                serviceIntent.putExtra(DeviceService.MAC_ADDRESS, device.address)
                context.startForegroundService(serviceIntent)
            }
        }
    }
}
