package com.oppzippy.openscq30

import android.app.Application
import android.content.Intent
import com.oppzippy.openscq30.features.autoconnect.AutoConnectService
import com.oppzippy.openscq30.features.preferences.Preferences
import dagger.hilt.android.HiltAndroidApp
import javax.inject.Inject

@HiltAndroidApp
class OpenSCQ30Application : Application() {

    @Inject
    lateinit var preferences: Preferences

    init {
        Native.initialize()
    }

    override fun onCreate() {
        super.onCreate()
        if (preferences.autoConnect) {
            startService(Intent(this, AutoConnectService::class.java))
        }
    }
}
