package com.oppzippy.openscq30

import com.oppzippy.openscq30.lib.Init

object Native {
    private var isInitialized = false

    @Synchronized
    fun initialize() {
        if (!isInitialized) {
            System.loadLibrary("openscq30_android")
            Init.logging()
            isInitialized = true
        }
    }
}
