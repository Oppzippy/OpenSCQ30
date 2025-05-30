package com.oppzippy.openscq30.ui

import android.app.Application
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetSlot
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetSlotDao
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
    private val quickPresetSlotDao: QuickPresetSlotDao,
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
                        quickPresetSlotDao = quickPresetSlotDao,
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
    parent: Job,
    private val deviceManager: DeviceConnectionManager,
    private val quickPresetSlotDao: QuickPresetSlotDao,
) : AutoCloseable {
    private val coroutineScope = CoroutineScope(Job(parent))
    private val quickPresetHandler = session.quickPresetHandler()
    private val device = deviceManager.device

    private val _quickPresetsFlow = MutableStateFlow(emptyList<QuickPreset>())
    val quickPresetsFlow = _quickPresetsFlow.asStateFlow()

    init {
        coroutineScope.launch { refreshQuickPresets() }
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

    val allSettingsFlow: Flow<List<Pair<String, Setting>>>
        get() = deviceManager.watchForChangeNotification.map {
            device.categories()
                .flatMap { device.settingsInCategory(it) }
                .mapNotNull { settingId ->
                    device.setting(settingId)?.let { setting -> Pair(settingId, setting) }
                }
        }

    val quickPresetSlots = quickPresetSlotDao.allNames(device.model())

    fun setQuickPresetSlot(index: Int, name: String?) {
        coroutineScope.launch {
            if (name != null) {
                quickPresetSlotDao.upsert(QuickPresetSlot(device.model(), index, name))
            } else {
                quickPresetSlotDao.delete(device.model(), index)
            }
        }
    }

    fun activateQuickPreset(name: String) {
        coroutineScope.launch {
            quickPresetHandler.activate(device, name)
            refreshQuickPresets()
        }
    }

    fun createQuickPreset(name: String) {
        coroutineScope.launch {
            quickPresetHandler.save(device, name)
            refreshQuickPresets()
        }
    }

    fun toggleQuickPresetSetting(name: String, settingId: String, enabled: Boolean) {
        coroutineScope.launch {
            quickPresetHandler.toggleField(device, name, settingId, enabled)
            // TODO either modify in place instead of re-fetching all presets, or fetch only the modified preset
            refreshQuickPresets()
        }
    }

    private suspend fun refreshQuickPresets() {
        _quickPresetsFlow.value = quickPresetHandler.quickPresets(device).sortedBy { it.name }
    }
}
