package com.oppzippy.openscq30.hilt

import android.content.Context
import com.oppzippy.openscq30.soundcoredevice.SoundcoreDeviceFactory
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
        return SoundcoreDeviceFactory(context)
    }
}