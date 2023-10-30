package com.oppzippy.openscq30.features.soundcoredevice.dagger

import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceFactoryImpl
import dagger.Binds
import dagger.Module
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent

@Module
@InstallIn(SingletonComponent::class)
abstract class SoundcoreDeviceFactoryModule {
    @Binds
    abstract fun bindSoundcoreDeviceFactory(soundcoreDeviceFactory: SoundcoreDeviceFactoryImpl): SoundcoreDeviceFactory
}
