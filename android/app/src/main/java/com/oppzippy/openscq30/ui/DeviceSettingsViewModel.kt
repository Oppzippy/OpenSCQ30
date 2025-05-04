package com.oppzippy.openscq30.ui

import android.app.Application
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch

@HiltViewModel
class DeviceSettingsViewModel @Inject constructor(
    private val application: Application,
    private val intentFactory: IntentFactory,
) : AndroidViewModel(application) {
    private val deviceServiceConnection =
        DeviceServiceConnection(unbind = { unbindDeviceService() })
    val uiDeviceState = deviceServiceConnection.connectionStatusFlow.asStateFlow()

    init {
        bindDeviceService()
    }

    override fun onCleared() {
        unbindDeviceService()
        stopServiceIfNotificationIsGone()
    }

    fun selectDevice(macAddress: String) {
        val intent = intentFactory(application, DeviceService::class.java)
        intent.putExtra(DeviceService.MAC_ADDRESS, macAddress)
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
            application.stopService(intentFactory(application, DeviceService::class.java))
            application.unbindService(deviceServiceConnection)
            deviceServiceConnection.onUnbind()
        } catch (_: IllegalArgumentException) {
            // service is not bound
        }
    }

    fun setSettingValues(settingValues: List<Pair<String, Value>>) {
        viewModelScope.launch {
            deviceServiceConnection.deviceManager?.device?.let { device ->
                device.setSettingValues(
                    settingValues.map { valuePair ->
                        SettingIdValuePair(valuePair.first, valuePair.second)
                    },
                )
            }
        }
    }

    fun getCategoriesFlow(): Flow<List<String>> = deviceServiceConnection.connectionStatusFlow.map {
        if (it is ConnectionStatus.Connected) {
            it.deviceManager.device.categories()
        } else {
            emptyList()
        }
    }

    fun getSettingsInCategoryFlow(categoryId: String): Flow<List<Pair<String, Setting>>> =
        deviceServiceConnection.deviceManager?.let { deviceManager ->
            deviceManager.watchForChangeNotification.map {
                deviceManager.device.settingsInCategory(categoryId).mapNotNull { settingId ->
                    deviceManager.device.setting(settingId)?.let { Pair(settingId, it) }
                }
            }
        } ?: emptyFlow()
}
