package com.oppzippy.openscq30.ui.deviceselection.composables

import android.Manifest
import android.os.Build
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.deviceselection.models.Screen

@Composable
fun DeviceSelection(
    devices: List<BluetoothDevice>,
    onRefreshDevices: () -> Unit = {},
    onDeviceSelected: (device: BluetoothDevice) -> Unit = {},
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
            ) {
                DeviceListing(
                    devices,
                    onRefreshClick = onRefreshDevices,
                    onDeviceClick = onDeviceSelected,
                    onInfoClick = {
                        navController.navigate(Screen.Info.route) {
                            launchSingleTop = true
                        }
                    },
                )
            }
        }
        composable("/info") {
            AppInfo(onBackClick = {
                navController.popBackStack()
            })
        }
    }
}
