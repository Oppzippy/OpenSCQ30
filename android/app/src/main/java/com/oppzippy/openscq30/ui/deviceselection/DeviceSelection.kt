package com.oppzippy.openscq30.ui.deviceselection

import android.Manifest
import android.content.Intent
import android.os.Build
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.ui.deviceselection.composables.AppInfo
import com.oppzippy.openscq30.ui.deviceselection.composables.DeviceListing
import com.oppzippy.openscq30.ui.deviceselection.composables.PermissionCheck
import com.oppzippy.openscq30.ui.deviceselection.models.Screen
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun DeviceSelectionRoot(
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

    Surface(
        modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background,
    ) {
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
}
