package com.oppzippy.openscq30.features.autoconnect

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import com.oppzippy.openscq30.features.preferences.Preferences
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class AutoStartReceiver @Inject constructor(
    private val preferences: Preferences,
) : BroadcastReceiver() {
    override fun onReceive(context: Context?, intent: Intent?) {
        Log.d("BootReceiver", "starting background service")
        if (preferences.autoConnect) {
            context?.applicationContext?.startService(
                Intent(context, AutoConnectService::class.java),
            )
        }
    }
}
