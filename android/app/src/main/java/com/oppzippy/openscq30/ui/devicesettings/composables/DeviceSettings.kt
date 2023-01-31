package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.devicesettings.composables.equalizer.EqualizerSettings
import com.oppzippy.openscq30.ui.devicesettings.composables.general.GeneralSettings
import com.oppzippy.openscq30.ui.devicesettings.models.EqualizerProfile
import com.oppzippy.openscq30.ui.devicesettings.models.Screen
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings() {
    val navController = rememberNavController()
    val navItems = listOf(
        Screen.General,
        Screen.Equalizer,
    )
    Scaffold(bottomBar = {
        NavigationBar {
            val navBarStackEntry by navController.currentBackStackEntryAsState()
            val currentDestination = navBarStackEntry?.destination
            navItems.forEach { screen ->
                NavigationBarItem(icon = { Icon(screen.icon, contentDescription = null) },
                    label = { Text(stringResource(screen.resourceId)) },
                    selected = currentDestination?.route == screen.route,
                    onClick = {
                        navController.navigate(screen.route) {
                            popUpTo(navController.graph.id)
                            launchSingleTop = true
                        }
                    })
            }
        }
    }) { innerPadding ->
        NavHost(
            navController = navController,
            startDestination = Screen.General.route,
            modifier = Modifier.padding(innerPadding)
        ) {
            composable(Screen.General.route) {
                GeneralSettings()
            }
            composable(Screen.Equalizer.route) {
                EqualizerSettings()
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        DeviceSettings()
    }
}
