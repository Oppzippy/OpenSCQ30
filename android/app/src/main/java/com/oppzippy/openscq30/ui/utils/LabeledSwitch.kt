package com.oppzippy.openscq30.ui.utils

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material3.Switch
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun LabeledSwitch(
    label: String,
    isChecked: Boolean,
    onCheckedChange: (value: Boolean) -> Unit,
    modifier: Modifier = Modifier,
    extraButtons: (@Composable () -> Unit)? = null,
) {
    Labeled(
        modifier = modifier
            .toggleable(
                value = isChecked,
                onValueChange = onCheckedChange,
                role = Role.Switch,
            ),
        label = label,
    ) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            if (extraButtons != null) {
                extraButtons()
            }
            Switch(
                checked = isChecked,
                onCheckedChange = null,
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewLabeledSwitch() {
    OpenSCQ30Theme {
        LabeledSwitch(label = "Switch", isChecked = true, onCheckedChange = {})
    }
}
