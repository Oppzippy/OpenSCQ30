package com.oppzippy.openscq30.ui

import android.app.Application
import android.util.Log
import android.widget.Toast
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.android.IntentFactory
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfile
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfileDao
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.features.statusnotification.storage.FeaturedSettingSlot
import com.oppzippy.openscq30.features.statusnotification.storage.FeaturedSettingSlotDao
import com.oppzippy.openscq30.features.statusnotification.storage.QuickPresetSlot
import com.oppzippy.openscq30.features.statusnotification.storage.QuickPresetSlotDao
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.ModifiableSelectCommandInner
import com.oppzippy.openscq30.lib.wrapper.QuickPreset
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.lib.wrapper.toValue
import com.oppzippy.openscq30.ui.utils.ToastHandler
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlin.math.roundToInt
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
    private val featuredSettingSlotDao: FeaturedSettingSlotDao,
    private val legacyEqualizerProfileDao: LegacyEqualizerProfileDao,
    val toastHandler: ToastHandler,
) : AndroidViewModel(application) {
    private val deviceServiceConnection =
        DeviceServiceConnection(unbind = { unbindDeviceService() })
    val connectionStatusFlow = deviceServiceConnection.connectionStatusFlow.asStateFlow()

    var deviceSettingsManager = MutableStateFlow<DeviceSettingsManager?>(null)
        private set

    companion object {
        const val TAG = "OpenSCQ30RootViewModel"
    }

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
                        featuredSettingSlotDao = featuredSettingSlotDao,
                        legacyEqualizerProfileDao = legacyEqualizerProfileDao,
                        toastHandler = toastHandler,
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
                TAG,
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
            Log.e(TAG, "failed to bind service", ex)
            unbindDeviceService()
        }
    }

    fun deselectDevice() {
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
    private val featuredSettingSlotDao: FeaturedSettingSlotDao,
    legacyEqualizerProfileDao: LegacyEqualizerProfileDao,
    private val toastHandler: ToastHandler,
) : AutoCloseable {
    private val coroutineScope = CoroutineScope(Job(parent))
    private val quickPresetHandler = session.quickPresetHandler()
    private val device = deviceManager.device

    private val _quickPresetsFlow = MutableStateFlow(emptyList<QuickPreset>())
    val quickPresetsFlow = _quickPresetsFlow.asStateFlow()

    companion object {
        const val TAG = "DeviceSettingsManager"
    }

    init {
        coroutineScope.launch { refreshQuickPresets() }
    }

    override fun close() {
        coroutineScope.cancel()
    }

    fun setSettingValues(settingValues: List<Pair<String, Value>>) {
        coroutineScope.launch {
            try {
                device.setSettingValues(
                    settingValues.map { (settingId, value) ->
                        SettingIdValuePair(settingId, value)
                    },
                )
            } catch (ex: OpenScq30Exception) {
                Log.e(TAG, "error setting values of settings", ex)
                toastHandler.add(R.string.error_changing_settings, Toast.LENGTH_SHORT)
            }
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

    val featuredSettingSlots = featuredSettingSlotDao.allSettingIds(device.model()).map { slots ->
        // Show a fixed number of slots
        List(2) { slots.getOrNull(it) }
    }

    fun setFeaturedSettingSlot(index: Int, settingId: String?) {
        coroutineScope.launch {
            if (settingId != null) {
                featuredSettingSlotDao.upsert(FeaturedSettingSlot(device.model(), index, settingId))
            } else {
                featuredSettingSlotDao.delete(device.model(), index)
            }
        }
    }

    val quickPresetSlots = quickPresetSlotDao.allNames(device.model()).map { slots ->
        // Show a fixed number of slots
        List(2) { slots.getOrNull(it) }
    }

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
            try {
                quickPresetHandler.activate(device, name)
            } catch (ex: OpenScq30Exception) {
                Log.e(TAG, "error activating quick preset", ex)
                toastHandler.add(R.string.error_activating_quick_preset, Toast.LENGTH_SHORT)
            }
            refreshQuickPresets()
        }
    }

    fun createQuickPreset(name: String) {
        coroutineScope.launch {
            try {
                quickPresetHandler.save(device, name)
            } catch (ex: OpenScq30Exception) {
                Log.e(TAG, "error saving quick preset", ex)
                toastHandler.add(R.string.error_saving_quick_preset, Toast.LENGTH_SHORT)
            }
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

    val legacyEqualizerProfilesFlow = legacyEqualizerProfileDao.all()

    fun migrateLegacyEqualizerProfile(legacyEqualizerProfile: LegacyEqualizerProfile) {
        val oldVolumeAdjustments = device.setting("volumeAdjustments") ?: return
        if (oldVolumeAdjustments !is Setting.EqualizerSetting) return

        coroutineScope.launch {
            try {
                device.setSettingValues(
                    listOf(
                        SettingIdValuePair(
                            "volumeAdjustments",
                            legacyEqualizerProfile.getVolumeAdjustments().map { (it * 10.0).roundToInt().toShort() }
                                .toValue(),
                        ),
                        SettingIdValuePair(
                            "customEqualizerProfile",
                            Value.ModifiableSelectCommand(
                                ModifiableSelectCommandInner.Add(legacyEqualizerProfile.name),
                            ),
                        ),
                        SettingIdValuePair("volumeAdjustments", oldVolumeAdjustments.toValue()),
                    ),
                )
            } catch (ex: OpenScq30Exception) {
                Log.e(TAG, "error migrating legacy equalizer profile", ex)
                toastHandler.add(R.string.error_migrating_legacy_profile, Toast.LENGTH_SHORT)
            }
        }
    }
}
