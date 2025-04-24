package com.oppzippy.openscq30.features.autoconnect

import android.bluetooth.BluetoothDevice
import android.content.Intent
import android.content.IntentFilter
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.features.preferences.Preferences
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class AutoConnectService : LifecycleService() {
    @Inject
    lateinit var preferences: Preferences

    @Inject
    lateinit var session: OpenScq30Session

    private lateinit var receiver: BluetoothConnectionReceiver

    override fun onCreate() {
        super.onCreate()
        receiver = BluetoothConnectionReceiver(
            preferences = preferences,
            session = session,
            coroutineScope = lifecycleScope,
        )
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
