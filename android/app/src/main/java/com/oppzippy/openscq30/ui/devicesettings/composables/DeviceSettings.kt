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
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.Value
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.ScreenInfo
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.importexport.ImportExportScreen
import com.oppzippy.openscq30.ui.importexport.ImportExportViewModel
import com.oppzippy.openscq30.ui.soundmode.SoundModeSettings
import kotlinx.coroutines.flow.Flow

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    uiState: UiDeviceState.Connected,
    onBack: () -> Unit = {},
    categoryIds: List<String>,
    getSettingFlow: (String) -> Flow<Setting?>,
    setSettings: suspend (List<Pair<String, Value>>) -> Unit,
) {
    val navController = rememberNavController()
    val listedScreens: MutableList<ScreenInfo> =
        categoryIds.map { Screen.SettingsCategory(it).screenInfo() }.toMutableList()
    listedScreens.add(Screen.ImportExport.screenInfo)

    // compose navigation does not allow us to use polymorphism with routes, so instead a mapping of
    // class path to route name is kept
    val routeNames = listedScreens.associate {
        Pair(it.baseRoute::class.qualifiedName!!, it.name.translated())
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
                    val title = resourceId ?: uiState.name
                    Text(title)
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
                SoundModeSettings(
                    categoryIds = categoryIds,
                    getSettingFlow = getSettingFlow,
                    setSettings = setSettings,
                )
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
