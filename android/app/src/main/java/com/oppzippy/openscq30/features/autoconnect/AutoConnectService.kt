package com.oppzippy.openscq30.features.autoconnect

import android.bluetooth.BluetoothDevice
import android.content.Intent
import android.content.IntentFilter
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.features.preferences.Preferences
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject
import kotlinx.coroutines.launch

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
        application.registerReceiver(
            receiver,
            IntentFilter(BluetoothDevice.ACTION_ACL_CONNECTED),
        )
        // Devices that connected before the receiver was registered (for example right after an app update, or when
        // the earbuds connected before boot completed) won't trigger ACTION_ACL_CONNECTED, so check for them now.
        lifecycleScope.launch {
            if (!preferences.autoConnect || DeviceService.isRunning.value) return@launch
            val device = connectedPairedDevices(applicationContext, session).firstOrNull {
                isAutoConnectCandidate(applicationContext, session, it.macAddress)
            }
            if (device != null && !DeviceService.isRunning.value) {
                startDeviceService(applicationContext, device.macAddress)
            }
        }
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
