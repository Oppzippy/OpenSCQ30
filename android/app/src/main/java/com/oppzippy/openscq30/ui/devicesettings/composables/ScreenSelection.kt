package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.devicesettings.Screen
import com.oppzippy.openscq30.ui.devicesettings.ScreenInfo
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun ScreenSelection(screens: List<ScreenInfo>, onNavigation: (Screen) -> Unit) {
    FlowRow(horizontalArrangement = Arrangement.SpaceBetween, maxItemsInEachRow = 2) {
        screens.forEach { screenInfo ->
            Column(
                modifier = Modifier
                    .clickable { onNavigation(screenInfo.baseRoute) }
                    .padding(16.dp)
                    .weight(1f),
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                Icon(screenInfo.icon, contentDescription = null)
                Text(screenInfo.name.translated())
            }
        }
    }
}

@Preview
@Composable
private fun PreviewScreenSelection() {
    OpenSCQ30Theme(dynamicColor = false) {
        ScreenSelection(
            screens = listOf(
                Screen.SettingsCategory("general").screenInfo(),
                Screen.SettingsCategory("equalizer").screenInfo(),
                Screen.SettingsCategory("buttonConfiguration").screenInfo(),
                Screen.ImportExport.screenInfo,
            ),
            onNavigation = {},
        )
    }
}
