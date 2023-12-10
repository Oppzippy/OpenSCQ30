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
import com.oppzippy.openscq30.lib.wrapper.DeviceState

@Composable
fun DeviceInfoScreen(deviceState: DeviceState) {
    LazyVerticalGrid(
        columns = GridCells.Fixed(2),
        modifier = Modifier.fillMaxSize(),
        verticalArrangement = Arrangement.Top,
        horizontalArrangement = Arrangement.Center,
        contentPadding = PaddingValues(horizontal = 8.dp, vertical = 4.dp),
        userScrollEnabled = true,
    ) {
        if (deviceState.firmwareVersion != null) {
            item { Text(stringResource(R.string.firmware_version)) }
            item { Text(deviceState.firmwareVersion.toString()) }
        }
        if (deviceState.serialNumber != null) {
            item { Text(stringResource(R.string.serial_number)) }
            item { Text(deviceState.serialNumber) }
        }
        if (deviceState.ageRange != null) {
            item { Text(stringResource(R.string.age_range)) }
            item { Text(deviceState.ageRange.toString()) }
        }
        item { Text(stringResource(R.string.device_profile)) }
        item { Text(deviceState.deviceProfile.toString()) }
    }
}
