package com.oppzippy.openscq30.ui.devicesettings.screens

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun Disconnected() {
    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center,
    ) {
        Text(stringResource(R.string.disconnected))
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewDisconnected() {
    OpenSCQ30Theme {
        Disconnected()
    }
}
