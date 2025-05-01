package com.oppzippy.openscq30

import com.oppzippy.openscq30.lib.bindings.LanguageIdentifier
import com.oppzippy.openscq30.lib.bindings.initNativeI18n
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

    private var isI18nInitialized = false

    @Synchronized
    fun initializeI18n(languageIdentifiers: List<LanguageIdentifier>) {
        if (!isI18nInitialized) {
            initNativeI18n(languageIdentifiers)
            isI18nInitialized = true
        }
    }
}
