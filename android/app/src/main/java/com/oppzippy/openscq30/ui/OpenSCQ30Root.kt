package com.oppzippy.openscq30.ui

import androidx.activity.compose.BackHandler
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.SizeTransform
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.ui.deviceselection.DeviceSelectionScreen
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsScreen
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.Loading

@Composable
fun OpenSCQ30Root(viewModel: OpenSCQ30RootViewModel = hiltViewModel()) {
    OpenSCQ30Theme {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background,
        ) {
            Box(Modifier.safeDrawingPadding()) {
                val connectionStatus by viewModel.connectionStatusFlow.collectAsState()

                val isConnected =
                    connectionStatus is ConnectionStatus.Connected || connectionStatus is ConnectionStatus.Connecting
                BackHandler(enabled = isConnected) {
                    viewModel.deselectDevice()
                }
                AnimatedContent(
                    targetState = isConnected,
                    transitionSpec = {
                        val widthDivisor = if (targetState) 2 else -2
                        slideInHorizontally { width -> width / widthDivisor } + fadeIn() togetherWith
                            slideOutHorizontally { width -> width / -widthDivisor } + fadeOut() using
                            SizeTransform(
                                clip = false,
                            )
                    },
                    label = "Selection to Settings animation",
                ) { animationIsConnected ->
                    if (animationIsConnected) {
                        val deviceSettings = viewModel.deviceSettingsManager.collectAsState().value
                        if (deviceSettings != null) {
                            DeviceSettingsScreen(
                                connectionStatus = connectionStatus,
                                onBack = { viewModel.deselectDevice() },
                                setSettingValues = { deviceSettings.setSettingValues(it) },
                                allSettingsFlow = deviceSettings.allSettingsFlow,
                                categoryIdsFlow = deviceSettings.categoryIdsFlow,
                                getSettingsInCategoryFlow = { deviceSettings.getSettingsInCategoryFlow(it) },
                                quickPresetSlotsFlow = deviceSettings.quickPresetSlots,
                                onQuickPresetSlotChange = { index, name ->
                                    deviceSettings.setQuickPresetSlot(
                                        index,
                                        name,
                                    )
                                },
                                quickPresetsFlow = deviceSettings.quickPresetsFlow,
                                activateQuickPreset = { deviceSettings.activateQuickPreset(it) },
                                createQuickPreset = { deviceSettings.createQuickPreset(it) },
                                toggleQuickPresetSetting = { name: String, settingId: String, enabled: Boolean ->
                                    deviceSettings.toggleQuickPresetSetting(name, settingId, enabled)
                                },
                            )
                        } else {
                            Loading()
                        }
                    } else {
                        DeviceSelectionScreen(
                            onDeviceSelected = { viewModel.selectDevice(it.macAddress) },
                        )
                    }
                }
            }
        }
    }
}
