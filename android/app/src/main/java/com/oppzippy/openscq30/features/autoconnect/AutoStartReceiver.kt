package com.oppzippy.openscq30.features.autoconnect

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import com.oppzippy.openscq30.features.preferences.Preferences

class AutoStartReceiver : BroadcastReceiver() {
    companion object {
        private const val TAG = "AutoStartReceiver"
    }

    override fun onReceive(context: Context?, intent: Intent?) {
        // MY_PACKAGE_REPLACED is needed as well, otherwise auto connect stops working after an app update until the
        // next reboot.
        val isStartupAction = intent?.action == Intent.ACTION_BOOT_COMPLETED ||
            intent?.action == Intent.ACTION_MY_PACKAGE_REPLACED
        if (context != null && isStartupAction) {
            val preferences = Preferences(context.applicationContext)
            if (preferences.autoConnect) {
                Log.d(TAG, "starting background service")
                context.applicationContext.startService(
                    Intent(context.applicationContext, AutoConnectService::class.java),
                )
            } else {
                Log.d(TAG, "not starting background service on boot since auto connect is disabled")
            }
        }
    }
}
