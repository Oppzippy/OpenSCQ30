package com.oppzippy.openscq30.features.autoconnect

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log

class AutoStartReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context?, intent: Intent?) {
        Log.d("BootReceiver", "starting background service")
        context?.applicationContext?.startService(
            Intent(context, AutoConnectService::class.java),
        )
    }
}
