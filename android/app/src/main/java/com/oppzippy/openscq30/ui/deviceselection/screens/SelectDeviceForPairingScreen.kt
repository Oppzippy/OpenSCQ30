package com.oppzippy.openscq30.ui.deviceselection.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.wrapper.ConnectionDescriptor
import com.oppzippy.openscq30.ui.deviceselection.components.AddDeviceCard
import com.oppzippy.openscq30.ui.utils.LabeledSwitch
import com.oppzippy.openscq30.ui.utils.Loading

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SelectDeviceForPairingScreen(
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
                    .padding(horizontal = 16.dp)
                    .fillMaxSize(),
                verticalArrangement = Arrangement.spacedBy(16.dp),
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
                        AddDeviceCard(
                            modifier = Modifier.clickable { onDescriptorSelected(descriptor) },
                            name = descriptor.name,
                            macAddress = descriptor.macAddress,
                        )
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
