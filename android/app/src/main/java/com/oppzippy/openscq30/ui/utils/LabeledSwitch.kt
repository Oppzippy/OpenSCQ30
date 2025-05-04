package com.oppzippy.openscq30.ui.utils

import androidx.compose.foundation.selection.toggleable
import androidx.compose.material3.Switch
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun LabeledSwitch(
    label: String,
    isChecked: Boolean,
    onCheckedChange: (value: Boolean) -> Unit,
    modifier: Modifier = Modifier,
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
        Switch(
            checked = isChecked,
            onCheckedChange = null,
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewLabeledSwitch() {
    OpenSCQ30Theme {
        LabeledSwitch(label = "Switch", isChecked = true, onCheckedChange = {})
    }
}
