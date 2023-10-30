package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.content.Context
import javax.inject.Inject

class SoundcoreDeviceCallbackHandlerFactoryImpl @Inject constructor() :
    SoundcoreDeviceCallbackHandlerFactory {
    override fun createSoundcoreDeviceCallbackHandler(context: Context): SoundcoreDeviceCallbackHandler {
        return SoundcoreDeviceCallbackHandler(context)
    }
}
