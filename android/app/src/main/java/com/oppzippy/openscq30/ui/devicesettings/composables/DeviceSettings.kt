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
import androidx.compose.ui.unit.dp
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.toRoute
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfile
import com.oppzippy.openscq30.features.soundcoredevice.service.ConnectionStatus
import com.oppzippy.openscq30.lib.bindings.translateCategoryId
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.wrapper.QuickPreset
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.ScreenInfo
import kotlinx.coroutines.flow.Flow

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    connectionStatus: ConnectionStatus.Connected,
    onBack: () -> Unit = {},
    setSettingValues: (settingValues: List<Pair<String, Value>>) -> Unit,
    categoryIdsFlow: Flow<List<String>>,
    allSettingsFlow: Flow<List<Pair<String, Setting>>>,
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
    val categoryIds by categoryIdsFlow.collectAsState(emptyList())

    val listedScreens: MutableList<ScreenInfo> =
        categoryIds.map { Screen.SettingsCategory(it).screenInfo() }.toMutableList()
    listedScreens.add(Screen.QuickPresets.screenInfo)
    listedScreens.add(Screen.StatusNotification.screenInfo)
    listedScreens.add(Screen.MigrateLegacyEqualizerProfiles.screenInfo)

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
                        val editQuickPreset = try {
                            backStackEntry.toRoute<Screen.EditQuickPreset>().name
                        } catch (_: Exception) {
                            null
                        }
                        val modelName = translateDeviceModel(connectionStatus.deviceManager.device.model())

                        Text(routeName ?: settingsCategory ?: editQuickPreset ?: modelName)
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
            modifier = Modifier
                .padding(innerPadding)
                .padding(horizontal = 8.dp),
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
                val settings by getSettingsInCategoryFlow(route.categoryId).collectAsState(emptyList())
                SettingPage(
                    settings = settings,
                    setSettings = setSettingValues,
                )
            }

            composable<Screen.QuickPresets> {
                val quickPresets by quickPresetsFlow.collectAsState(emptyList())
                QuickPresetsPage(
                    quickPresets = quickPresets,
                    onActivate = activateQuickPreset,
                    onCreate = createQuickPreset,
                    onEdit = { navController.navigate(Screen.EditQuickPreset(it)) },
                    onDelete = deleteQuickPreset,
                )
            }

            composable<Screen.EditQuickPreset> { backStackEntry ->
                val route = backStackEntry.toRoute<Screen.EditQuickPreset>()
                val quickPresets by quickPresetsFlow.collectAsState(emptyList())
                val quickPreset = quickPresets.find { it.name == route.name }
                val settings by allSettingsFlow.collectAsState(emptyList())
                if (quickPreset != null) {
                    EditQuickPresetPage(
                        settings = settings.toMap(),
                        quickPreset = quickPreset,
                        onToggleSetting = toggleQuickPresetSetting,
                        onLoadCurrentSettings = { onQuickPresetLoadCurrentSettings(quickPreset.name) },
                    )
                }
            }

            composable<Screen.StatusNotification> {
                val settings by allSettingsFlow.collectAsState(emptyList())
                val featuredSettingSlots by featuredSettingSlotsFlow.collectAsState(emptyList())
                val quickPresets by quickPresetsFlow.collectAsState(emptyList())
                val quickPresetSlots by quickPresetSlotsFlow.collectAsState(emptyList())
                StatusNotificationPage(
                    settingIds = settings.map { it.first },
                    featuredSettingSlots = featuredSettingSlots,
                    onFeaturedSettingSlotChange = onFeaturedSettingSlotChange,
                    quickPresets = quickPresets.map { it.name },
                    quickPresetSlots = quickPresetSlots,
                    onQuickPresetSlotChange = onQuickPresetSlotChange,
                )
            }

            composable<Screen.MigrateLegacyEqualizerProfiles> {
                val legacyEqualizerProfiles by legacyEqualizerProfilesFlow.collectAsState(emptyList())
                MigrateLegacyEqualizerProfilesPage(
                    legacyEqualizerProfiles = legacyEqualizerProfiles,
                    onMigrateLegacyEqualizerProfile = onMigrateLegacyEqualizerProfile,
                )
            }
        }
    }
}
