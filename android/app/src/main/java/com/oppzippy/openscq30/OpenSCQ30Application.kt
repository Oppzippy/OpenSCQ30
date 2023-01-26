package com.oppzippy.openscq30

import android.app.Application
import com.oppzippy.openscq30.lib.Init
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class OpenSCQ30Application : Application() {
    init {
        Native.initialize()
    }
}