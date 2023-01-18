package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.PopUpToBuilder
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    ambientSoundMode: AmbientSoundMode,
    noiseCancelingMode: NoiseCancelingMode,
    equalizerProfile: EqualizerProfile,
    equalizerValues: List<Byte>,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit,
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit,
    onEqualizerProfileChange: (equalizerProfile: EqualizerProfile) -> Unit,
    onEqualizerValueChange: (index: Int, changedValue: Byte) -> Unit
) {
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
                GeneralSettings(
                    ambientSoundMode = ambientSoundMode,
                    noiseCancelingMode = noiseCancelingMode,
                    onAmbientSoundModeChange = onAmbientSoundModeChange,
                    onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                )
            }
            composable(Screen.Equalizer.route) {
                EqualizerSettings(
                    profile = equalizerProfile,
                    equalizerValues = equalizerValues,
                    onProfileChange = onEqualizerProfileChange,
                    onEqualizerValueChange = onEqualizerValueChange,
                )
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        var ambientSoundMode by remember { mutableStateOf(AmbientSoundMode.Normal) }
        var noiseCancelingMode by remember { mutableStateOf(NoiseCancelingMode.Transport) }
        var equalizerProfile by remember { mutableStateOf(EqualizerProfile.Acoustic) }
        var equalizerValues by remember { mutableStateOf(listOf<Byte>(0, 0, 0, 0, 0, 0, 0, 0)) }
        DeviceSettings(ambientSoundMode = ambientSoundMode,
            noiseCancelingMode = noiseCancelingMode,
            onAmbientSoundModeChange = { ambientSoundMode = it },
            onNoiseCancelingModeChange = { noiseCancelingMode = it },
            equalizerProfile = equalizerProfile,
            equalizerValues = equalizerValues,
            onEqualizerProfileChange = { equalizerProfile = it },
            onEqualizerValueChange = { changedIndex, changedValue ->
                equalizerValues = equalizerValues.mapIndexed { index, value ->
                    if (index == changedIndex) {
                        changedValue
                    } else {
                        value
                    }
                }
            })
    }
}
