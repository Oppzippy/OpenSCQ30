package com.oppzippy.openscq30.features.soundcoredevice

import android.content.Context
import com.oppzippy.openscq30.BuildConfig
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.demo.DemoSoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceFactoryImpl
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ActivityRetainedComponent
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.android.scopes.ActivityRetainedScoped

@Module
@InstallIn(ActivityRetainedComponent::class)
object SoundcoreDeviceFactoryModule {
    @Provides
    @ActivityRetainedScoped
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