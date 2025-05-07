package com.oppzippy.openscq30.ui.devicesettings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceConnectionManager
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import dagger.assisted.Assisted
import dagger.assisted.AssistedFactory
import dagger.assisted.AssistedInject
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch

@HiltViewModel(assistedFactory = DeviceSettingsViewModel.DeviceSettingsViewModelFactory::class)
class DeviceSettingsViewModel @AssistedInject constructor(
    @Assisted private val deviceManager: DeviceConnectionManager,
) : ViewModel() {
    @AssistedFactory
    interface DeviceSettingsViewModelFactory {
        fun create(deviceManager: DeviceConnectionManager): DeviceSettingsViewModel
    }

    fun setSettingValues(settingValues: List<Pair<String, Value>>) {
        viewModelScope.launch {
            deviceManager.device.setSettingValues(
                settingValues.map { (settingId, value) ->
                    SettingIdValuePair(settingId, value)
                },
            )
        }
    }

    fun getCategoryIdsFlow(): Flow<List<String>> = deviceManager.watchForChangeNotification.map {
        deviceManager.device.categories()
    }

    fun getSettingsInCategoryFlow(categoryId: String): Flow<List<Pair<String, Setting>>> =
        deviceManager.watchForChangeNotification.map {
            deviceManager.device.settingsInCategory(categoryId).mapNotNull { settingId ->
                deviceManager.device.setting(settingId)?.let { Pair(settingId, it) }
            }
        }

    fun getAllSettingIdsFlow(categoryId: String): Flow<List<Pair<String, List<String>>>> =
        deviceManager.watchForChangeNotification.map {
            deviceManager.device.categories().map { categoryId ->
                Pair(categoryId, deviceManager.device.settingsInCategory(categoryId))
            }
        }
}
