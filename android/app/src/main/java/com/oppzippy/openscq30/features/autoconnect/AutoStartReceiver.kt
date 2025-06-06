package com.oppzippy.openscq30.features.autoconnect

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import com.oppzippy.openscq30.features.preferences.Preferences

class AutoStartReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context?, intent: Intent?) {
        if (context != null && intent?.action == Intent.ACTION_BOOT_COMPLETED) {
            val preferences = Preferences(context.applicationContext)
            if (preferences.autoConnect) {
                Log.d("AutoStartReceiver", "starting background service")
                context.applicationContext.startService(
                    Intent(context.applicationContext, AutoConnectService::class.java),
                )
            } else {
                Log.d(
                    "AutoStartReceiver",
                    "not starting background service on boot since auto connect is disabled",
                )
            }
        }
    }
}
