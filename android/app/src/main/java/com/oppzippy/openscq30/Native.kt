package com.oppzippy.openscq30

import com.oppzippy.openscq30.lib.bindings.initNativeLogging

object Native {
    private var isInitialized = false

    @Synchronized
    fun initialize() {
        if (!isInitialized) {
            System.loadLibrary("openscq30_android")
            initNativeLogging()
            isInitialized = true
        }
    }
}
