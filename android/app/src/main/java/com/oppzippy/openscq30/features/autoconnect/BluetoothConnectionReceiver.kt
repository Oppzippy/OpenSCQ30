package com.oppzippy.openscq30.features.autoconnect

import android.bluetooth.BluetoothDevice
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import androidx.core.content.IntentCompat
import com.oppzippy.openscq30.features.preferences.Preferences
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

class BluetoothConnectionReceiver(
    private val preferences: Preferences,
    private val session: OpenScq30Session,
    private val coroutineScope: CoroutineScope,
) : BroadcastReceiver() {
    companion object {
        private const val TAG = "BluetoothConnectionReceiver"
    }

    override fun onReceive(context: Context?, intent: Intent?) {
        if (!preferences.autoConnect) {
            Log.w(
                TAG,
                "Got device connected event, but auto connect is disabled. This service should not be running.",
            )
            return
        }

        if (context != null &&
            intent != null &&
            intent.action == BluetoothDevice.ACTION_ACL_CONNECTED
        ) {
            val device = IntentCompat.getParcelableExtra(
                intent,
                BluetoothDevice.EXTRA_DEVICE,
                BluetoothDevice::class.java,
            )
            if (device != null) {
                coroutineScope.launch {
                    if (isAutoConnectCandidate(context, session, device.address)) {
                        startDeviceService(context, device.address)
                    }
                }
            }
        }
    }
}

fun startDeviceService(context: Context, macAddress: String) {
    Log.d("AutoConnect", "auto connecting to $macAddress")
    val serviceIntent = Intent(context, DeviceService::class.java)
    serviceIntent.putExtra(DeviceService.INTENT_EXTRA_MAC_ADDRESS, macAddress)
    context.startForegroundService(serviceIntent)
}
