package com.oppzippy.openscq30.features.soundcoredevice

import android.content.Context
import com.oppzippy.openscq30.BuildConfig
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.demo.DemoSoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceFactoryImpl
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ServiceComponent
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.android.scopes.ServiceScoped

@Module
@InstallIn(ServiceComponent::class)
object SoundcoreDeviceFactoryModule {
    @Provides
    @ServiceScoped
    fun provideSoundcoreDeviceFactory(
        @ApplicationContext context: Context,
    ): SoundcoreDeviceFactory {
        return if (BuildConfig.IS_DEMO_MODE) {
            DemoSoundcoreDeviceFactory()
        } else {
            SoundcoreDeviceFactoryImpl(context)
        }
    }
}
