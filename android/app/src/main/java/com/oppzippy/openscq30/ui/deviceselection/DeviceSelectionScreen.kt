@file:OptIn(ExperimentalMaterial3Api::class)

package com.oppzippy.openscq30.ui.deviceselection

import androidx.activity.compose.LocalActivity
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.deviceModels
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import com.oppzippy.openscq30.lib.wrapper.PairedDevice
import com.oppzippy.openscq30.ui.deviceselection.composables.DeviceSelection
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel

@Composable
fun DeviceSelectionScreen(
    onDeviceSelected: (device: PairedDevice) -> Unit,
    viewModel: DeviceSelectionViewModel = hiltViewModel(),
) {
    val activity = LocalActivity.current!!
    when (val state = viewModel.state.collectAsState().value) {
        DeviceSelectionState.Loading -> Box { Text(stringResource(R.string.loading)) }

        is DeviceSelectionState.Connect -> {
            DeviceSelection(
                devices = state.devices,
                onRefreshDevices = {},
                onDeviceClick = { onDeviceSelected(it) },
                onUnpair = { viewModel.unpair(it) },
                onAddDeviceClick = {
                    viewModel.state.value = DeviceSelectionState.SelectModelForPairing
                },
            )
        }

        is DeviceSelectionState.SelectModelForPairing -> SelectModelForPairing(
            onModelSelected = { viewModel.selectModel(it) },
        )

        is DeviceSelectionState.SelectDeviceForPairing -> SelectDeviceForPairing(
            model = state.model,
            isDemoMode = state.isDemoMode,
            devices = state.devices,
            onDemoModeChange = { viewModel.setDemoMode(state, it) },
            onDescriptorSelected = { viewModel.pair(activity, it) },
        )
    }
}

@Composable
fun SelectModelForPairing(onModelSelected: (String) -> Unit) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(text = stringResource(id = R.string.select_device_model))
                },
            )
        },
        content = { innerPadding ->
            LazyColumn(
                modifier = Modifier
                    .padding(innerPadding)
                    .fillMaxSize(),
            ) {
                items(deviceModels()) { model ->
                    Column(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable { onModelSelected(model) }
                            .padding(horizontal = 8.dp, vertical = 8.dp),
                    ) {
                        Text(text = translateDeviceModel(model))
                        Text(text = model, color = MaterialTheme.colorScheme.secondary)
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
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(text = stringResource(id = R.string.select_x, translateDeviceModel(model)))
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
                    CheckboxWithLabel(
                        stringResource(R.string.demo_mode),
                        isDemoMode,
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

