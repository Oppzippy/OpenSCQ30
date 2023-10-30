package com.oppzippy.openscq30.features.soundcoredevice.dagger

import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceCallbackHandlerFactory
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceCallbackHandlerFactoryImpl
import dagger.Binds
import dagger.Module
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent

@Module
@InstallIn(SingletonComponent::class)
abstract class SoundcoreDeviceCallbackHandlerFactoryModule {
    @Binds
    abstract fun bindSoundcoreDeviceCallbackHandlerFactory(soundcoreDeviceCallbackHandlerFactory: SoundcoreDeviceCallbackHandlerFactoryImpl): SoundcoreDeviceCallbackHandlerFactory
}
