package com.oppzippy.openscq30.ui.devicesettings.composables.equalizer

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.LocalTextStyle
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun ProfileSelectionRow(name: String, volumeAdjustments: List<Double>?) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(name)
        if (volumeAdjustments != null) {
            val lineHeightSp = LocalTextStyle.current.fontSize
            val lineHeightDp = with(LocalDensity.current) {
                lineHeightSp.toDp()
            }
            EqualizerLine(
                values = volumeAdjustments,
                width = 80.dp,
                height = lineHeightDp,
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewProfileSelectionRow() {
    OpenSCQ30Theme {
        ProfileSelectionRow(
            name = "Test Profile",
            volumeAdjustments = listOf(0.0, 1.0, 5.0, 10.0, -10.0, -5.0, -1.0, 0.0),
        )
    }
}
