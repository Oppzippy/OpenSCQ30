package com.oppzippy.openscq30.ui

import android.app.Application
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.QuickPreset
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.job
import kotlinx.coroutines.launch

@HiltViewModel
class OpenSCQ30RootViewModel @Inject constructor(
    private val session: OpenScq30Session,
    private val application: Application,
    private val intentFactory: IntentFactory,
) : AndroidViewModel(application) {
    private val deviceServiceConnection =
        DeviceServiceConnection(unbind = { unbindDeviceService() })
    val connectionStatusFlow = deviceServiceConnection.connectionStatusFlow.asStateFlow()

    var deviceSettingsManager = MutableStateFlow<DeviceSettingsManager?>(null)
        private set

    init {
        bindDeviceService()
        viewModelScope.launch {
            connectionStatusFlow.collect {
                deviceSettingsManager.value?.close()
                if (it is ConnectionStatus.Connected) {
                    deviceSettingsManager.value = DeviceSettingsManager(
                        session = session,
                        deviceManager = it.deviceManager,
                        parent = coroutineContext.job,
                    )
                } else {
                    deviceSettingsManager.value = null
                }
            }
        }
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
}

class DeviceSettingsManager(
    session: OpenScq30Session,
    private val deviceManager: DeviceConnectionManager,
    parent: Job,
) : AutoCloseable {
    private val coroutineScope = CoroutineScope(Job(parent))
    private val quickPresetHandler = session.quickPresetHandler()
    private val device = deviceManager.device

    private val _quickPresetsFlow = MutableStateFlow(emptyList<QuickPreset>())
    val quickPresetsFlow = _quickPresetsFlow.asStateFlow()

    init {
        coroutineScope.launch {
            _quickPresetsFlow.value = quickPresetHandler.quickPresets(device)
        }
    }

    override fun close() {
        coroutineScope.cancel()
    }

    fun setSettingValues(settingValues: List<Pair<String, Value>>) {
        coroutineScope.launch {
            device.setSettingValues(
                settingValues.map { (settingId, value) ->
                    SettingIdValuePair(settingId, value)
                },
            )
        }
    }

    val categoryIdsFlow: Flow<List<String>>
        get() = deviceManager.watchForChangeNotification.map {
            device.categories()
        }

    fun getSettingsInCategoryFlow(categoryId: String): Flow<List<Pair<String, Setting>>> =
        deviceManager.watchForChangeNotification.map {
            device.settingsInCategory(categoryId).mapNotNull { settingId ->
                device.setting(settingId)?.let { Pair(settingId, it) }
            }
        }

    fun activateQuickPreset(name: String) {
        coroutineScope.launch { quickPresetHandler.activate(device, name) }
    }

    fun createQuickPreset(name: String) {
        coroutineScope.launch { quickPresetHandler.save(device, name, emptyMap()) }
    }

    fun toggleQuickPresetSetting(name: String, settingId: String, enabled: Boolean) {
        val quickPreset = _quickPresetsFlow.value.find { it.name == name } ?: return
        val newSettings = quickPreset.settings.map { field ->
            if (field.settingId == settingId) {
                if (enabled) {
                    field.copy(value = device.setting(field.settingId)?.toValue())
                } else {
                    field.copy(value = null)
                }
            } else {
                field
            }
        }.filter { it.value != null }.associate { Pair(it.settingId, it.value!!) }
        coroutineScope.launch {
            quickPresetHandler.save(device, name, newSettings)
            _quickPresetsFlow.value = quickPresetHandler.quickPresets(device)
        }
    }
}
