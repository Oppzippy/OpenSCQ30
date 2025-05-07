package com.oppzippy.openscq30.ui.devicesettings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.QuickPreset
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import dagger.assisted.Assisted
import dagger.assisted.AssistedFactory
import dagger.assisted.AssistedInject
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch

@HiltViewModel(assistedFactory = DeviceSettingsViewModel.DeviceSettingsViewModelFactory::class)
class DeviceSettingsViewModel @AssistedInject constructor(
    session: OpenScq30Session,
    @Assisted private val deviceManager: DeviceConnectionManager,
) : ViewModel() {
    private val quickPresetHandler = session.quickPresetHandler()
    private val device = deviceManager.device

    private val _quickPresetsFlow = MutableStateFlow(emptyList<QuickPreset>())
    val quickPresetsFlow = _quickPresetsFlow.asStateFlow()

    init {
        viewModelScope.launch {
            _quickPresetsFlow.value = quickPresetHandler.quickPresets(device)
        }
    }

    @AssistedFactory
    interface DeviceSettingsViewModelFactory {
        fun create(deviceManager: DeviceConnectionManager): DeviceSettingsViewModel
    }

    fun setSettingValues(settingValues: List<Pair<String, Value>>) {
        viewModelScope.launch {
            device.setSettingValues(
                settingValues.map { (settingId, value) ->
                    SettingIdValuePair(settingId, value)
                },
            )
        }
    }

    fun getCategoryIdsFlow(): Flow<List<String>> = deviceManager.watchForChangeNotification.map {
        device.categories()
    }

    fun getSettingsInCategoryFlow(categoryId: String): Flow<List<Pair<String, Setting>>> =
        deviceManager.watchForChangeNotification.map {
            device.settingsInCategory(categoryId).mapNotNull { settingId ->
                device.setting(settingId)?.let { Pair(settingId, it) }
            }
        }

    fun activateQuickPreset(name: String) {
        viewModelScope.launch { quickPresetHandler.activate(device, name) }
    }

    fun createQuickPreset(name: String) {
        viewModelScope.launch { quickPresetHandler.save(device, name, emptyMap()) }
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
        viewModelScope.launch {
            quickPresetHandler.save(device, name, newSettings)
            _quickPresetsFlow.value = quickPresetHandler.quickPresets(device)
        }
    }
}
