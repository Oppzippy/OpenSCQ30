package com.oppzippy.openscq30.ui.deviceinfo

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.SoundcoreDeviceState

@Composable
fun DeviceInfoScreen(deviceState: SoundcoreDeviceState) {
    LazyVerticalGrid(
        columns = GridCells.Fixed(2),
        modifier = Modifier.fillMaxSize(),
        verticalArrangement = Arrangement.Top,
        horizontalArrangement = Arrangement.Center,
        contentPadding = PaddingValues(horizontal = 8.dp, vertical = 4.dp),
    ) {
        if (deviceState.leftFirmwareVersion != null) {
            item { Text(stringResource(R.string.firmware_version)) }
            item {
                if (deviceState.rightFirmwareVersion != null) {
                    Text("${deviceState.leftFirmwareVersion}, ${deviceState.rightFirmwareVersion}")
                } else {
                    Text(deviceState.leftFirmwareVersion.toString())
                }
            }
        }
        if (deviceState.serialNumber != null) {
            item { Text(stringResource(R.string.serial_number)) }
            item { Text(deviceState.serialNumber) }
        }
        if (deviceState.ageRange != null) {
            item { Text(stringResource(R.string.age_range)) }
            item { Text(deviceState.ageRange.value().toString()) }
        }
        item { Text(stringResource(R.string.feature_flags)) }
        item { Text(deviceState.featureFlags.bits().toString()) }
    }
}
