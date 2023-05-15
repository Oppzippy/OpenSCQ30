package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp

@Composable
fun ProfileSelectionRow(name: String, volumeAdjustments: List<Byte>?) {
    Row(
        modifier = Modifier
            .fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(name)
        if (volumeAdjustments != null) {
            val lineHeightSp = MaterialTheme.typography.bodyMedium.fontSize
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
