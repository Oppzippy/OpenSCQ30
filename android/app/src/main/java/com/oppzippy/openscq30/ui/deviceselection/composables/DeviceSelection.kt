package com.oppzippy.openscq30.ui.deviceselection.composables

import android.Manifest
import android.os.Build
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import com.oppzippy.openscq30.ui.deviceselection.models.Screen
import com.oppzippy.openscq30.ui.settings.SettingsPage
import com.oppzippy.openscq30.ui.utils.PermissionCheck

@Composable
fun DeviceSelection(
    devices: List<PairedDevice>,
    onRefreshDevices: () -> Unit = {},
    onDeviceClick: (device: PairedDevice) -> Unit = {},
    onUnpair: (PairedDevice) -> Unit = {},
    onAddDeviceClick: () -> Unit,
) {
    val navController = rememberNavController()

    val bluetoothPermission = if (Build.VERSION.SDK_INT >= 31) {
        Manifest.permission.BLUETOOTH_CONNECT
    } else {
        Manifest.permission.BLUETOOTH
    }

    NavHost(
        navController = navController,
        startDestination = Screen.Home.route,
    ) {
        composable("/") {
            PermissionCheck(
                permission = bluetoothPermission,
                prompt = stringResource(R.string.bluetooth_permission_is_required),
                onPermissionGranted = onRefreshDevices,
            ) {
                DeviceListing(
                    devices,
                    onRefreshClick = onRefreshDevices,
                    onDeviceClick = onDeviceClick,
                    onUnpair = onUnpair,
                    onInfoClick = {
                        navController.navigate(Screen.Info.route) {
                            launchSingleTop = true
                        }
                    },
                    onSettingsClick = {
                        navController.navigate(Screen.Settings.route)
                    },
                    onAddDeviceClick = onAddDeviceClick,
                )
            }
        }
        composable("/info") {
            AppInfo(
                onBackClick = {
                    navController.popBackStack()
                },
            )
        }
        composable("/settings") {
            SettingsPage(
                onBackClick = {
                    navController.popBackStack()
                },
            )
        }
    }
}
