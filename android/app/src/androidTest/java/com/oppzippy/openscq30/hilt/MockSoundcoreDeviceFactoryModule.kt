package com.oppzippy.openscq30.hilt

import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.dagger.SoundcoreDeviceConnectorModule
import dagger.Module
import dagger.Provides
import dagger.hilt.components.SingletonComponent
import dagger.hilt.testing.TestInstallIn
import io.mockk.mockk
import javax.inject.Singleton

@Module
@TestInstallIn(
    components = [SingletonComponent::class],
    replaces = [SoundcoreDeviceConnectorModule::class],
)
object MockSoundcoreDeviceFactoryModule {
    @Provides
    @Singleton
    fun provideSoundcoreDeviceFactoryProvider(): SoundcoreDeviceConnector {
        return mockk()
    }
}
