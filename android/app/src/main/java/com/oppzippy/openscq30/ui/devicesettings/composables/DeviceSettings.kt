import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.lib.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.composables.SoundModeSettings
import com.oppzippy.openscq30.ui.devicesettings.models.UiDeviceState
import com.oppzippy.openscq30.ui.equalizer.EqualizerSettings
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceSettings(
    uiState: UiDeviceState.Connected,
    onBack: () -> Unit = {},
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode) -> Unit = {},
    onEqualizerConfigurationChange: (equalizerConfiguration: EqualizerConfiguration) -> Unit = {},
) {
    val navController = rememberNavController()
    val navItems = listOf(
        Screen.General,
        Screen.Equalizer,
    )
    Scaffold(
        topBar = {
            TopAppBar(title = {
                Text(uiState.name)
            }, navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(
                        imageVector = Icons.Filled.ArrowBack,
                        contentDescription = stringResource(R.string.back),
                    )
                }
            })
        },
        bottomBar = {
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
        },
    ) { innerPadding ->
        NavHost(
            navController = navController,
            startDestination = Screen.General.route,
            modifier = Modifier.padding(innerPadding)
        ) {
            composable(Screen.General.route) {
                SoundModeSettings(
                    ambientSoundMode = uiState.deviceState.ambientSoundMode(),
                    noiseCancelingMode = uiState.deviceState.noiseCancelingMode(),
                    onAmbientSoundModeChange = onAmbientSoundModeChange,
                    onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                )
            }
            composable(Screen.Equalizer.route) {
                EqualizerSettings(
                    uiState = uiState,
                    onEqualizerConfigurationChange = onEqualizerConfigurationChange,
                )
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        DeviceSettings(
            uiState = UiDeviceState.Connected(
                "Soundcore Q30",
                "00:00:00:00:00:00",
                SoundcoreDeviceState(
                    AmbientSoundMode.Normal,
                    NoiseCancelingMode.Indoor,
                    EqualizerConfiguration(PresetEqualizerProfile.SoundcoreSignature),
                ),
            )
        )
    }
}
