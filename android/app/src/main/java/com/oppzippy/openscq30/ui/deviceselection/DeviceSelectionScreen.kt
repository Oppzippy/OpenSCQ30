@file:OptIn(ExperimentalMaterial3Api::class)

package com.oppzippy.openscq30.ui.deviceselection

import android.Manifest
import android.os.Build
import androidx.activity.compose.BackHandler
import androidx.activity.compose.LocalActivity
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

    PermissionCheck(
        permission = bluetoothPermission,
        prompt = stringResource(R.string.bluetooth_permission_is_required),
        onPermissionGranted = { viewModel.refreshDevices() },
    ) {
        BackHandler(enabled = viewModel.hasBack) { viewModel.back() }

        when (val pageState = viewModel.pageState.collectAsState().value) {
            DeviceSelectionPage.Loading -> {
                Loading()
            }

            is DeviceSelectionPage.Connect -> {
                DeviceListing(
                    devices = pageState.devices,
                    onDeviceClick = { onDeviceSelected(it) },
                    onUnpair = { viewModel.unpair(it) },
                    onAddDeviceClick = { viewModel.pageState.value = DeviceSelectionPage.SelectModelForPairing },
                    onRefreshClick = { viewModel.refreshDevices() },
                    onSettingsClick = { viewModel.pageState.value = DeviceSelectionPage.Settings },
                    onInfoClick = { viewModel.pageState.value = DeviceSelectionPage.Info },
                )
            }

            is DeviceSelectionPage.SelectDeviceForPairing -> {
                val activity = LocalActivity.current!!
                SelectDeviceForPairing(
                    model = pageState.model,
                    isDemoMode = pageState.isDemoMode,
                    devices = pageState.devices,
                    onDemoModeChange = { viewModel.setDemoMode(pageState, it) },
                    onDescriptorSelected = {
                        viewModel.pair(
                            activity,
                            PairedDevice(
                                macAddress = it.macAddress,
                                model = pageState.model,
                                isDemo = pageState.isDemoMode,
                            ),
                        )
                    },
                    onBackClick = { viewModel.back() },
                )
            }

            DeviceSelectionPage.SelectModelForPairing -> {
                SelectModelForPairing(
                    onModelSelected = { viewModel.selectModel(it) },
                    onBackClick = { viewModel.back() },
                )
            }

            DeviceSelectionPage.Info -> {
                AppInfo(onBackClick = { viewModel.back() })
            }

            DeviceSelectionPage.Settings -> {
                SettingsPage(onBackClick = { viewModel.back() })
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
    devices: List<ConnectionDescriptor>,
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
            }
        },
    )
}
