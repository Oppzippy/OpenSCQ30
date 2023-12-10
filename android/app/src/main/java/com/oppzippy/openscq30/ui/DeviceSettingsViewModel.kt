package com.oppzippy.openscq30.ui

import android.app.Application
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.SoundModes
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
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

    init {
        bindDeviceService()
    }

    override fun onCleared() {
        unbindDeviceService()
        stopServiceIfNotificationIsGone()
    }

    fun selectDevice(bluetoothDevice: BluetoothDevice) {
        val intent = intentFactory(application, DeviceService::class.java)
        intent.putExtra(DeviceService.MAC_ADDRESS, bluetoothDevice.address)
        application.startForegroundService(intent)
        bindDeviceService()
    }

    private fun stopServiceIfNotificationIsGone() {
        if (!DeviceService.doesNotificationExist(application)) {
            Log.i(
                "OpenSCQ30Root",
                "Stopping service since main activity is exiting and notification is not shown.",
            )
            deselectDevice()
        }
    }

    private fun bindDeviceService() {
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

    fun deselectDevice() {
        application.stopService(intentFactory(application, DeviceService::class.java))
        unbindDeviceService()
    }

    private fun unbindDeviceService() {
        try {
            application.unbindService(deviceServiceConnection)
        } catch (_: IllegalArgumentException) {
            // service is not bound
        }
    }

    fun setSoundModes(
        ambientSoundMode: AmbientSoundMode? = null,
        noiseCancelingMode: NoiseCancelingMode? = null,
        transparencyMode: TransparencyMode? = null,
        customNoiseCanceling: UByte? = null,
    ) {
        deviceServiceConnection.uiDeviceStateFlow.value.let { state ->
            if (state is UiDeviceState.Connected) {
                state.deviceState.soundModes?.let { soundModes ->
                    deviceServiceConnection.setSoundModes(
                        SoundModes(
                            ambientSoundMode ?: soundModes.ambientSoundMode,
                            noiseCancelingMode ?: soundModes.noiseCancelingMode,
                            transparencyMode ?: soundModes.transparencyMode,
                            customNoiseCanceling ?: soundModes.customNoiseCanceling,
                        ),
                    )
                }
            }
        }
    }

    fun setEqualizerConfiguration(equalizerConfiguration: EqualizerConfiguration) {
        deviceServiceConnection.setEqualizerConfiguration(equalizerConfiguration)
    }
}
