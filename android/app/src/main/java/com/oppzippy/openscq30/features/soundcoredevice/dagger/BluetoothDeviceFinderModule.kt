package com.oppzippy.openscq30.features.soundcoredevice.dagger

import com.oppzippy.openscq30.features.soundcoredevice.impl.BluetoothDeviceFinder
import com.oppzippy.openscq30.features.soundcoredevice.impl.BluetoothDeviceFinderImpl
import dagger.Binds
import dagger.Module
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent

@Module
@InstallIn(SingletonComponent::class)
abstract class BluetoothDeviceFinderModule {
    @Binds
    abstract fun bindBluetoothDeviceFinderModule(bluetoothDeviceFinder: BluetoothDeviceFinderImpl): BluetoothDeviceFinder
}
