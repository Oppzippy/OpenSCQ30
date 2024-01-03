package com.oppzippy.openscq30.ui.utils

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun <T> LabeledRadioButtonGroup(
    modifier: Modifier = Modifier,
    selectedValue: T,
    values: LinkedHashMap<T, String>,
    onValueChange: (value: T) -> Unit,
) {
    Column(modifier.selectableGroup()) {
        values.forEach { (value, text) ->
            LabeledRadioButton(text = text, selected = selectedValue == value, onClick = {
                onValueChange(value)
            })
        }
    }
}

@Composable
private fun LabeledRadioButton(text: String, selected: Boolean, onClick: () -> Unit) {
    Row(
        Modifier
            .fillMaxWidth()
            .selectable(selected = selected, onClick = onClick, role = Role.RadioButton)
            .padding(horizontal = 2.dp, vertical = 2.dp),
    ) {
        RadioButton(selected = selected, onClick = null)
        Text(
            text = text,
            modifier = Modifier.padding(start = 8.dp),
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewLabeledRadioButtonGroup() {
    OpenSCQ30Theme {
        LabeledRadioButtonGroup(
            selectedValue = 1,
            values = linkedMapOf(
                Pair(1, "Item 1"),
                Pair(2, "Item 2"),
                Pair(3, "Item 3"),
            ),
            onValueChange = {},
        )
    }
}
