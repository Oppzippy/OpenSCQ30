package com.oppzippy.openscq30

import android.app.Application
import android.content.Intent
import com.oppzippy.openscq30.features.autoconnect.AutoConnectService
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class OpenSCQ30Application : Application() {
    init {
        Native.initialize()
    }

    override fun onCreate() {
        super.onCreate()
        startService(Intent(this, AutoConnectService::class.java))
    }
}
