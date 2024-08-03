package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.toRoute
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundModeCycle
import com.oppzippy.openscq30.lib.wrapper.CustomButtonModel
import com.oppzippy.openscq30.lib.wrapper.EqualizerConfiguration
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingMode
import com.oppzippy.openscq30.lib.wrapper.NoiseCancelingModeType
import com.oppzippy.openscq30.lib.wrapper.TransparencyMode
import com.oppzippy.openscq30.lib.wrapper.TransparencyModeType
import com.oppzippy.openscq30.ui.buttonactions.ButtonActionSelection
import com.oppzippy.openscq30.ui.buttonactions.ButtonActions
import com.oppzippy.openscq30.ui.deviceinfo.DeviceInfoScreen
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.ScreenInfo
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.equalizer.EqualizerSettings
import com.oppzippy.openscq30.ui.importexport.ImportExportScreen
import com.oppzippy.openscq30.ui.importexport.ImportExportViewModel
import com.oppzippy.openscq30.ui.quickpresets.QuickPresetScreen
import com.oppzippy.openscq30.ui.soundmode.NoiseCancelingType
import com.oppzippy.openscq30.ui.soundmode.SoundModeSettings

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    uiState: UiDeviceState.Connected,
    onBack: () -> Unit = {},
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onAmbientSoundModeCycleChange: (ambientSoundMode: AmbientSoundModeCycle) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: UByte) -> Unit = {},
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
    onCustomButtonModelChange: (CustomButtonModel) -> Unit = {},
) {
    val navController = rememberNavController()
    val listedScreens = ArrayList<ScreenInfo>()
    if (uiState.deviceState.deviceProfile.soundMode != null) {
        listedScreens.add(Screen.General.screenInfo)
    }
    if (uiState.deviceState.deviceProfile.numEqualizerChannels > 0) {
        listedScreens.add(Screen.Equalizer.screenInfo)
    }
    listedScreens.add(Screen.QuickPresets.screenInfo)
    if (uiState.deviceState.deviceProfile.hasCustomButtonModel) {
        listedScreens.add(Screen.ButtonActions.screenInfo)
    }
    listedScreens.add(Screen.DeviceInfo.screenInfo)
    listedScreens.add(Screen.ImportExport.screenInfo)

    // compose navigation does not allow us to use polymorphism with routes, so instead a mapping of
    // class path to route name is kept
    val routeNames = listedScreens.associate {
        Pair(it.baseRoute::class.qualifiedName!!, it.nameResourceId)
    }

    // In order to avoid multiple instance of the view model being created, it needs to be created
    // outside of the nav controller's scope
    val importExportViewModel = hiltViewModel<ImportExportViewModel>()

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    val route =
                        navController.currentBackStackEntryAsState().value?.destination?.route
                    val routeWithoutArgs = route?.substringBefore("?")
                    val resourceId = routeNames[routeWithoutArgs]
                    val title = resourceId?.let { stringResource(it) } ?: uiState.name
                    Text(title)
                },
                navigationIcon = {
                    IconButton(onClick = {
                        val isAtTopOfStack = !navController.popBackStack()
                        if (isAtTopOfStack) {
                            onBack()
                        }
                    }) {
                        Icon(
                            imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = stringResource(R.string.back),
                        )
                    }
                },
            )
        },
    ) { innerPadding ->
        NavHost(
            navController = navController,
            startDestination = Screen.ScreenSelection,
            modifier = Modifier.padding(innerPadding),
            enterTransition = {
                slideInHorizontally { width -> width / 2 } + fadeIn()
            },
            exitTransition = {
                slideOutHorizontally { width -> -width / 2 } + fadeOut()
            },
            popEnterTransition = {
                slideInHorizontally { width -> -width / 2 } + fadeIn()
            },
            popExitTransition = {
                slideOutHorizontally { width -> width / 2 } + fadeOut()
            },
        ) {
            composable<Screen.ScreenSelection> {
                ScreenSelection(screens = listedScreens, onNavigation = { screen ->
                    navController.navigate(screen) {
                        popUpTo(Screen.ScreenSelection)
                        launchSingleTop = true
                    }
                })
            }
            uiState.deviceState.soundModes?.let { soundModes ->
                composable<Screen.General> {
                    val soundModeProfile =
                        uiState.deviceState.deviceProfile.soundMode ?: return@composable
                    SoundModeSettings(
                        soundModes = soundModes,
                        ambientSoundModeCycle = uiState.deviceState.ambientSoundModeCycle,
                        hasTransparencyModes = soundModeProfile.transparencyModeType == TransparencyModeType.Custom,
                        noiseCancelingType = when (soundModeProfile.noiseCancelingModeType) {
                            NoiseCancelingModeType.None -> NoiseCancelingType.None
                            NoiseCancelingModeType.Basic -> NoiseCancelingType.Normal
                            NoiseCancelingModeType.Custom -> NoiseCancelingType.Custom
                        },
                        onAmbientSoundModeChange = onAmbientSoundModeChange,
                        onTransparencyModeChange = onTransparencyModeChange,
                        onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                        onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
                        onAmbientSoundModeCycleChange = onAmbientSoundModeCycleChange,
                    )
                }
            }
            composable<Screen.Equalizer> {
                EqualizerSettings(
                    uiState = uiState,
                    onEqualizerConfigurationChange = onEqualizerConfigurationChange,
                )
            }
            composable<Screen.QuickPresets> {
                QuickPresetScreen(
                    deviceProfile = uiState.deviceState.deviceProfile,
                    deviceBleServiceUuid = uiState.deviceBleServiceUuid,
                )
            }
            uiState.deviceState.customButtonModel?.let { buttonModel ->
                composable<Screen.ButtonActions> {
                    ButtonActionSelection(
                        buttonActions = ButtonActions(
                            buttonModel.leftSingleClick.actionOrNull(),
                            buttonModel.leftDoubleClick.connectedActionOrNull(),
                            buttonModel.leftLongPress.connectedActionOrNull(),
                            buttonModel.rightSingleClick.actionOrNull(),
                            buttonModel.rightDoubleClick.connectedActionOrNull(),
                            buttonModel.rightLongPress.connectedActionOrNull(),
                        ),
                        onChange = {
                            onCustomButtonModelChange(it.toCustomButtonModel(buttonModel))
                        },
                    )
                }
            }
            composable<Screen.DeviceInfo> {
                DeviceInfoScreen(deviceState = uiState.deviceState)
            }

            composable<Screen.ImportExport> { backStackEntry ->
                val route = backStackEntry.toRoute<Screen.ImportExport>()
                ImportExportScreen(
                    viewModel = importExportViewModel,
                    index = route.index,
                    onIndexChange = { targetIndex ->
                        if (route.index < targetIndex) {
                            navController.navigate(Screen.ImportExport(targetIndex))
                        } else if (route.index > targetIndex) {
                            while (true) {
                                val current =
                                    navController.currentBackStackEntry?.toRoute<Screen.ImportExport>()
                                if (current != null && current.index > targetIndex) {
                                    navController.popBackStack()
                                } else {
                                    break
                                }
                            }
                        }
                    },
                )
            }
        }
    }
}
