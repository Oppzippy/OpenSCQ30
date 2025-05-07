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
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.toRoute
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.lib.bindings.translateCategoryId
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.ui.devicesettings.DeviceSettingsViewModel
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.ScreenInfo

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    connectionStatus: ConnectionStatus.Connected,
    onBack: () -> Unit = {},
    viewModel: DeviceSettingsViewModel = hiltViewModel(
        creationCallback = { factory: DeviceSettingsViewModel.DeviceSettingsViewModelFactory ->
            factory.create(
                connectionStatus.deviceManager,
            )
        },
    ),
) {
    val categoryIds by viewModel.getCategoryIdsFlow().collectAsState(emptyList())

    val listedScreens: MutableList<ScreenInfo> =
        categoryIds.map { Screen.SettingsCategory(it).screenInfo() }.toMutableList()
    listedScreens.add(Screen.QuickPresets.screenInfo)

    // compose navigation does not allow us to use polymorphism with routes, so instead a mapping of
    // class path to route name is kept
    val routeNames = listedScreens.associate {
        Pair(it.baseRoute::class.qualifiedName!!, it.name.translated())
    }

    val navController = rememberNavController()
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    val backStackEntry = navController.currentBackStackEntryAsState().value
                    if (backStackEntry != null) {
                        val routeName =
                            backStackEntry.destination.route?.let { routeNames[it] }
                        val settingsCategory = try {
                            translateCategoryId(backStackEntry.toRoute<Screen.SettingsCategory>().categoryId)
                        } catch (_: Exception) {
                            null
                        }
                        val modelName = translateDeviceModel(connectionStatus.deviceManager.device.model())

                        Text(routeName ?: settingsCategory ?: modelName)
                    }
                },
                navigationIcon = {
                    IconButton(
                        onClick = {
                            val isAtTopOfStack = !navController.popBackStack()
                            if (isAtTopOfStack) {
                                onBack()
                            }
                        },
                    ) {
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
                ScreenSelection(
                    screens = listedScreens,
                    onNavigation = { screen ->
                        navController.navigate(screen) {
                            popUpTo(Screen.ScreenSelection)
                            launchSingleTop = true
                        }
                    },
                )
            }
            composable<Screen.SettingsCategory> { backStackEntry ->
                val route = backStackEntry.toRoute<Screen.SettingsCategory>()
                val settings by viewModel.getSettingsInCategoryFlow(route.categoryId).collectAsState(emptyList())
                SettingPage(
                    settings = settings,
                    setSettings = { viewModel.setSettingValues(it) },
                )
            }

            composable<Screen.QuickPresets> {
                val quickPresets by viewModel.quickPresetsFlow.collectAsState()
                QuickPresetsPage(
                    quickPresets = quickPresets,
                    onActivate = { viewModel.activateQuickPreset(it) },
                    onToggleSetting = { name: String, settingId: String, isEnabled: Boolean ->
                        viewModel.toggleQuickPresetSetting(
                            name,
                            settingId,
                            isEnabled,
                        )
                    },
                    onCreate = { viewModel.createQuickPreset(it) },
                )
            }
        }
    }
}
