package com.oppzippy.openscq30.features.soundcoredevice.dagger

import android.content.Context
import com.oppzippy.openscq30.BuildConfig
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.demo.DemoSoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.impl.BluetoothDeviceFinder
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceCallbackHandlerFactory
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceConnectorImpl
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDeviceFactory
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ServiceComponent
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.android.scopes.ServiceScoped

@Module
@InstallIn(ServiceComponent::class)
object SoundcoreDeviceConnectorModule {
    @Provides
    @ServiceScoped
    fun provideSoundcoreDeviceFactory(
        @ApplicationContext context: Context,
        bluetoothDeviceFinder: BluetoothDeviceFinder,
        callbackHandlerFactory: SoundcoreDeviceCallbackHandlerFactory,
        deviceFactory: SoundcoreDeviceFactory,
    ): SoundcoreDeviceConnector {
        return if (BuildConfig.IS_DEMO_MODE) {
            DemoSoundcoreDeviceConnector()
        } else {
            SoundcoreDeviceConnectorImpl(
                context,
                bluetoothDeviceFinder,
                callbackHandlerFactory,
                deviceFactory,
            )
        }
    }
}
