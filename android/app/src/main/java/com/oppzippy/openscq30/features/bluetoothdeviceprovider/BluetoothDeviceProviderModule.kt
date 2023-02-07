package com.oppzippy.openscq30.features.bluetoothdeviceprovider

import android.content.Context
import com.oppzippy.openscq30.BuildConfig
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
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
        return if (BuildConfig.IS_DEMO_MODE) {
            DemoBluetoothDeviceProvider()
        } else {
            BluetoothDeviceProviderImpl(context)
        }
    }
}