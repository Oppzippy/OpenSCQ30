package com.oppzippy.openscq30.ui.quickpresets.composables

import androidx.compose.material3.Tab
import androidx.compose.material3.TabRow
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun QuickPresetSelection(selectedIndex: Int, onSelectedIndexChange: (index: Int) -> Unit) {
    TabRow(selectedTabIndex = selectedIndex) {
        for (i in 0..1) {
            Tab(
                selected = selectedIndex == i,
                onClick = { onSelectedIndexChange(i) },
                text = { Text(stringResource(R.string.quick_preset_number, i + 1)) },
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewQuickPresetSelection() {
    OpenSCQ30Theme {
        QuickPresetSelection(selectedIndex = 0, onSelectedIndexChange = {})
    }
}
