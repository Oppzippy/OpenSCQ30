package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.Composable
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfile
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
    allSettingsFlow: Flow<List<Pair<String, Setting>>>,
    categoryIdsFlow: Flow<List<String>>,
    getSettingsInCategoryFlow: (categoryId: String) -> Flow<List<Pair<String, Setting>>>,
    quickPresetSlotsFlow: Flow<List<String?>>,
    onQuickPresetSlotChange: (Int, String?) -> Unit,
    quickPresetsFlow: Flow<List<QuickPreset>>,
    activateQuickPreset: (name: String) -> Unit,
    createQuickPreset: (name: String) -> Unit,
    deleteQuickPreset: (name: String) -> Unit,
    toggleQuickPresetSetting: (name: String, settingId: String, enabled: Boolean) -> Unit,
    featuredSettingSlotsFlow: Flow<List<String?>>,
    onFeaturedSettingSlotChange: (Int, String?) -> Unit,
    onQuickPresetLoadCurrentSettings: (String) -> Unit,
    legacyEqualizerProfilesFlow: Flow<List<LegacyEqualizerProfile>>,
    onMigrateLegacyEqualizerProfile: (LegacyEqualizerProfile) -> Unit,
) {
    when (connectionStatus) {
        is ConnectionStatus.Connected -> {
            DeviceSettings(
                connectionStatus = connectionStatus,
                onBack = onBack,
                setSettingValues = setSettingValues,
                allSettingsFlow = allSettingsFlow,
                categoryIdsFlow = categoryIdsFlow,
                getSettingsInCategoryFlow = getSettingsInCategoryFlow,
                quickPresetSlotsFlow = quickPresetSlotsFlow,
                onQuickPresetSlotChange = onQuickPresetSlotChange,
                quickPresetsFlow = quickPresetsFlow,
                activateQuickPreset = activateQuickPreset,
                createQuickPreset = createQuickPreset,
                deleteQuickPreset = deleteQuickPreset,
                toggleQuickPresetSetting = toggleQuickPresetSetting,
                featuredSettingSlotsFlow = featuredSettingSlotsFlow,
                onFeaturedSettingSlotChange = onFeaturedSettingSlotChange,
                onQuickPresetLoadCurrentSettings = onQuickPresetLoadCurrentSettings,
                legacyEqualizerProfilesFlow = legacyEqualizerProfilesFlow,
                onMigrateLegacyEqualizerProfile = onMigrateLegacyEqualizerProfile,
            )
        }

        ConnectionStatus.AwaitingConnection -> Loading()
        is ConnectionStatus.Connecting -> Loading()
        ConnectionStatus.Disconnected -> Disconnected()
    }
}
