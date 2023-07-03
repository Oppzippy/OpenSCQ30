package com.oppzippy.openscq30.ui

import android.app.Application
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.asStateFlow
import javax.inject.Inject

@HiltViewModel
class DeviceSettingsViewModel @Inject constructor(
    private val application: Application,
    private val intentFactory: IntentFactory,
) : AndroidViewModel(application) {
    private val deviceServiceConnection =
        DeviceServiceConnection(unbind = { unbindDeviceService() })
    val uiDeviceState = deviceServiceConnection.uiDeviceStateFlow.asStateFlow()

    override fun onCleared() {
        unbindDeviceService()
    }

    fun selectDevice(bluetoothDevice: BluetoothDevice) {
        val intent = intentFactory(application, DeviceService::class.java)
        intent.putExtra(DeviceService.MAC_ADDRESS, bluetoothDevice.address)
        application.startForegroundService(intent)
        bindDeviceService()
    }

    fun deselectDevice() {
        application.stopService(intentFactory(application, DeviceService::class.java))
        unbindDeviceService()
    }

    fun bindDeviceService() {
        try {
            application.bindService(
                intentFactory(application, DeviceService::class.java),
                deviceServiceConnection,
                0,
            )
        } catch (ex: SecurityException) {
            Log.e("RootViewModel", "failed to bind service", ex)
            unbindDeviceService()
        }
    }

    fun unbindDeviceService() {
        try {
            application.unbindService(deviceServiceConnection)
        } catch (_: IllegalArgumentException) {
            // service is not bound
        }
    }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        deviceServiceConnection.setAmbientSoundMode(ambientSoundMode)
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        deviceServiceConnection.setNoiseCancelingMode(noiseCancelingMode)
    }

    fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        deviceServiceConnection.setEqualizerConfiguration(equalizerConfiguration)
    }

    fun stopServiceIfNotificationIsGone() {
        if (!deviceServiceConnection.doesNotificationExist()) {
            Log.i(
                "OpenSCQ30Root",
                "Stopping service since main activity is exiting and notification is not shown.",
            )
            deselectDevice()
        }
    }
}
