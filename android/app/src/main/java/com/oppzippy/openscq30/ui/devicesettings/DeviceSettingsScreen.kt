package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.Composable
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.lib.wrapper.QuickPreset
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.ui.devicesettings.composables.DeviceSettings
import com.oppzippy.openscq30.ui.devicesettings.composables.Disconnected
import com.oppzippy.openscq30.ui.utils.Loading
import kotlinx.coroutines.flow.Flow

@Composable
fun DeviceSettingsScreen(
    onBack: () -> Unit = {},
    connectionStatus: ConnectionStatus,
    setSettingValues: (settingValues: List<Pair<String, Value>>) -> Unit,
    categoryIdsFlow: Flow<List<String>>,
    getSettingsInCategoryFlow: (categoryId: String) -> Flow<List<Pair<String, Setting>>>,
    quickPresetsFlow: Flow<List<QuickPreset>>,
    activateQuickPreset: (name: String) -> Unit,
    createQuickPreset: (name: String) -> Unit,
    toggleQuickPresetSetting: (name: String, settingId: String, enabled: Boolean) -> Unit,
) {
    when (connectionStatus) {
        is ConnectionStatus.Connected -> {
            DeviceSettings(
                connectionStatus = connectionStatus,
                onBack = onBack,
                setSettingValues = setSettingValues,
                categoryIdsFlow = categoryIdsFlow,
                getSettingsInCategoryFlow = getSettingsInCategoryFlow,
                quickPresetsFlow = quickPresetsFlow,
                activateQuickPreset = activateQuickPreset,
                createQuickPreset = createQuickPreset,
                toggleQuickPresetSetting = toggleQuickPresetSetting,
            )
        }

        ConnectionStatus.AwaitingConnection -> Loading()
        is ConnectionStatus.Connecting -> Loading()
        ConnectionStatus.Disconnected -> Disconnected()
    }
}
