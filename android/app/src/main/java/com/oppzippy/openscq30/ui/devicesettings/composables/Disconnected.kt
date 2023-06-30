package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
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
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .fillMaxHeight(),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center,
    ) {
        Text(stringResource(R.string.disconnected))
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        Disconnected()
    }
}
