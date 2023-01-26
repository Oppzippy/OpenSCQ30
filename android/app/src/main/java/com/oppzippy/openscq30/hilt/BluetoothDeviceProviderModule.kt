package com.oppzippy.openscq30.hilt

import android.content.Context
import com.oppzippy.openscq30.ui.deviceselection.models.BluetoothDeviceProvider
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ActivityRetainedComponent
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.android.scopes.ActivityRetainedScoped

@Module
@InstallIn(ActivityRetainedComponent::class)
object BluetoothDeviceProviderModule {
    @Provides
    @ActivityRetainedScoped
    fun provideBluetoothDeviceProvider(
        @ApplicationContext context: Context,
    ): BluetoothDeviceProvider {
        return BluetoothDeviceProvider(context)
    }
}