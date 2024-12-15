package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun PairDeviceButton(modifier: Modifier = Modifier, text: String, onClick: () -> Unit) {
    Button(modifier = modifier, onClick = onClick) {
        Text(text)
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewPairDeviceButton() {
    OpenSCQ30Theme {
        PairDeviceButton(onClick = {}, text = "Pair Device")
    }
}
