package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.layout.*
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.oppzippy.openscq30.R


@OptIn(ExperimentalPermissionsApi::class)
@Composable
fun PermissionCheck(
    permission: String, prompt: String, children: @Composable () -> Unit
) {
    val permissionState = rememberPermissionState(permission)

    if (!permissionState.status.isGranted) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.Center,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(
                modifier = Modifier.fillMaxHeight(),
                verticalArrangement = Arrangement.Center,
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                Text(prompt)
                Button(onClick = { permissionState.launchPermissionRequest() }) {
                    Text(stringResource(R.string.request_permission))
                }
            }
        }
    } else {
        children()
    }
}
