package com.oppzippy.openscq30.features.autoconnect

import android.bluetooth.BluetoothDevice
import android.content.Intent
import android.content.IntentFilter
import androidx.lifecycle.LifecycleService
import dagger.hilt.android.AndroidEntryPoint

@AndroidEntryPoint
class AutoConnectService : LifecycleService() {
    private val receiver = BluetoothConnectionReceiver()

    override fun onCreate() {
        super.onCreate()
        // TODO check if any devices are currently connected, since they could have connected before we registered the receiver.
        application.registerReceiver(
            receiver,
            IntentFilter(BluetoothDevice.ACTION_ACL_CONNECTED),
        )
    }

    override fun onDestroy() {
        super.onDestroy()
        application.unregisterReceiver(receiver)
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)
        return START_STICKY
    }
}
