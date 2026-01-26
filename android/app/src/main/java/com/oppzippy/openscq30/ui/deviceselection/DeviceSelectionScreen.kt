@file:OptIn(ExperimentalMaterial3Api::class)

package com.oppzippy.openscq30.ui.deviceselection

import android.Manifest
import android.os.Build
import androidx.activity.compose.LocalActivity
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.hilt.lifecycle.viewmodel.compose.hiltViewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.toRoute
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.deviceModels
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import com.oppzippy.openscq30.ui.deviceselection.composables.AppInfo
import com.oppzippy.openscq30.ui.deviceselection.composables.DeviceListing
import com.oppzippy.openscq30.ui.settings.SettingsPage
import com.oppzippy.openscq30.ui.utils.LabeledSwitch
import com.oppzippy.openscq30.ui.utils.Loading
import com.oppzippy.openscq30.ui.utils.PermissionCheck
import kotlinx.serialization.Serializable

@Composable
fun DeviceSelectionScreen(
    onDeviceSelected: (device: PairedDevice) -> Unit,
    viewModel: DeviceSelectionViewModel = hiltViewModel(),
) {
    val bluetoothPermission = if (Build.VERSION.SDK_INT >= 31) {
        Manifest.permission.BLUETOOTH_CONNECT
    } else {
        Manifest.permission.BLUETOOTH
    }

    val navController = rememberNavController()

    PermissionCheck(
        permission = bluetoothPermission,
        prompt = stringResource(R.string.bluetooth_permission_is_required),
        onPermissionGranted = { viewModel.refreshPairedDevices() },
    ) {
        NavHost(
            navController = navController,
            startDestination = Screen.Connect,
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
            composable<Screen.Connect> {
                val activity = LocalActivity.current!!

                DeviceListing(
                    devices = viewModel.pairedDevices.collectAsState().value,
                    onDeviceClick = { onDeviceSelected(it) },
                    onUnpair = { viewModel.unpair(activity, it) },
                    onAddDeviceClick = { navController.navigate(Screen.SelectModelForPairing) },
                    onRefreshClick = { viewModel.refreshPairedDevices() },
                    onSettingsClick = { navController.navigate(Screen.Settings) },
                    onInfoClick = { navController.navigate(Screen.Info) },
                )
            }

            composable<Screen.SelectDeviceForPairing> { backStackEntry ->
                val screen = backStackEntry.toRoute<Screen.SelectDeviceForPairing>()
                val activity = LocalActivity.current!!

                var isDemoMode by remember { mutableStateOf(false) }
                val devices = remember { mutableStateOf<List<ConnectionDescriptor>?>(null) }
                LaunchedEffect(screen.model, isDemoMode) {
                    devices.value = viewModel.listDevices(screen.model, isDemoMode)
                }

                val devicesValue = devices.value
                SelectDeviceForPairing(
                    model = screen.model,
                    isDemoMode = isDemoMode,
                    devices = devicesValue,
                    onDemoModeChange = { isDemoMode = it },
                    onDescriptorSelected = {
                        viewModel.pair(
                            activity = activity,
                            pairedDevice = PairedDevice(
                                macAddress = it.macAddress,
                                model = screen.model,
                                isDemo = isDemoMode,
                            ),
                            onPaired = {
                                navController.popBackStack<Screen.Connect>(false)
                            },
                        )
                    },
                    onBackClick = { navController.popBackStack() },
                )
            }

            composable<Screen.SelectModelForPairing> {
                SelectModelForPairing(
                    onModelSelected = { navController.navigate(Screen.SelectDeviceForPairing(it)) },
                    onBackClick = { navController.popBackStack() },
                )
            }

            composable<Screen.Info> {
                AppInfo(onBackClick = { navController.popBackStack() })
            }

            composable<Screen.Settings> {
                SettingsPage(onBackClick = { navController.popBackStack() })
            }
        }
    }
}

@Composable
fun SelectModelForPairing(onModelSelected: (String) -> Unit, onBackClick: () -> Unit) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(text = stringResource(id = R.string.select_device_model))
                },
                navigationIcon = {
                    IconButton(onClick = onBackClick) {
                        Icon(
                            imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = stringResource(R.string.back),
                        )
                    }
                },
            )
        },
        content = { innerPadding ->
            Column(
                modifier = Modifier
                    .padding(innerPadding)
                    .fillMaxSize(),
            ) {
                var searchQuery by remember { mutableStateOf("") }
                TextField(
                    modifier = Modifier.fillMaxWidth(),
                    value = searchQuery,
                    onValueChange = { searchQuery = it },
                    label = { Text(stringResource(R.string.search)) },
                )
                LazyColumn(
                    modifier = Modifier
                        .testTag("modelList")
                        .fillMaxSize(),
                ) {
                    val filteredDeviceModels = deviceModels()
                        .map { Pair(it, translateDeviceModel(it)) }
                        .filter { (model, name) ->
                            model.contains(searchQuery, true) || name.contains(searchQuery, true)
                        }
                    if (filteredDeviceModels.isNotEmpty()) {
                        items(filteredDeviceModels) { (model, name) ->
                            Column(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .clickable { onModelSelected(model) }
                                    .padding(horizontal = 8.dp, vertical = 8.dp),
                            ) {
                                Text(text = name)
                                Text(text = model, color = MaterialTheme.colorScheme.secondary)
                            }
                        }
                    } else {
                        item {
                            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                                Text(stringResource(R.string.no_items_found))
                            }
                        }
                    }
                }
            }
        },
    )
}

@Composable
fun SelectDeviceForPairing(
    model: String,
    isDemoMode: Boolean,
    devices: List<ConnectionDescriptor>?,
    onDemoModeChange: (Boolean) -> Unit,
    onDescriptorSelected: (ConnectionDescriptor) -> Unit,
    onBackClick: () -> Unit,
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(text = stringResource(id = R.string.select_x, translateDeviceModel(model)))
                },
                navigationIcon = {
                    IconButton(onClick = onBackClick) {
                        Icon(
                            imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = stringResource(R.string.back),
                        )
                    }
                },
            )
        },
        content = { innerPadding ->
            LazyColumn(
                modifier = Modifier
                    .padding(innerPadding)
                    .fillMaxSize(),
            ) {
                item {
                    LabeledSwitch(
                        label = stringResource(R.string.demo_mode),
                        isChecked = isDemoMode,
                        onCheckedChange = { onDemoModeChange(it) },
                    )
                }
                if (devices != null) {
                    items(devices) { descriptor ->
                        Column(
                            modifier = Modifier
                                .fillMaxWidth()
                                .clickable { onDescriptorSelected(descriptor) }
                                .padding(horizontal = 8.dp, vertical = 8.dp),
                        ) {
                            Text(text = descriptor.name)
                            Text(text = descriptor.macAddress, color = MaterialTheme.colorScheme.secondary)
                        }
                    }
                } else {
                    item {
                        Loading()
                    }
                }
            }
        },
    )
}

@Serializable
private sealed class Screen {
    @Serializable
    data object Connect : Screen()

    @Serializable
    data object SelectModelForPairing : Screen()

    @Serializable
    data class SelectDeviceForPairing(val model: String) : Screen()

    @Serializable
    data object Info : Screen()

    @Serializable
    data object Settings : Screen()
}
