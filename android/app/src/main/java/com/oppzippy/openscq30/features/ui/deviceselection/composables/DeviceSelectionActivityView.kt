package com.oppzippy.openscq30.features.ui.deviceselection.composables

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.ui.deviceselection.models.Screen
import com.oppzippy.openscq30.features.ui.info.AppInfo
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun DeviceSelectionActivityView(
    bluetoothDeviceProvider: BluetoothDeviceProvider
) {
    val navController = rememberNavController()

    OpenSCQ30Theme {
        Surface(
            modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background,
        ) {
            NavHost(
                navController = navController,
                startDestination = Screen.Home.route,
            ) {
                composable("/") {
                    DeviceSelectionPermissionCheck(
                        bluetoothDeviceProvider = bluetoothDeviceProvider,
                        onInfoClick = {
                            navController.navigate(Screen.Info.route) {
                                launchSingleTop = true
                            }
                        },
                    )
                }
                composable("/info") {
                    AppInfo(
                        onBackClick = {
                            navController.popBackStack()
                        }
                    )
                }
            }
        }
    }
}
