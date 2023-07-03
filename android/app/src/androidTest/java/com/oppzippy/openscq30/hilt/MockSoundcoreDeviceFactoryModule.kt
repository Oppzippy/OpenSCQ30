package com.oppzippy.openscq30.hilt

import com.oppzippy.openscq30.features.soundcoredevice.SoundcoreDeviceFactoryModule
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import dagger.Module
import dagger.Provides
import dagger.hilt.components.SingletonComponent
import dagger.hilt.testing.TestInstallIn
import io.mockk.mockk
import javax.inject.Singleton

@Module
@TestInstallIn(
    components = [SingletonComponent::class],
    replaces = [SoundcoreDeviceFactoryModule::class],
)
object MockSoundcoreDeviceFactoryModule {
    @Provides
    @Singleton
    fun provideSoundcoreDeviceFactoryProvider(): SoundcoreDeviceFactory {
        return mockk()
    }
}
