package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.content.Context

interface SoundcoreDeviceCallbackHandlerFactory {
    fun createSoundcoreDeviceCallbackHandler(context: Context): SoundcoreDeviceCallbackHandler
}
