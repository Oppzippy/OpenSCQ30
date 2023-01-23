package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun <T> Dropdown(
    value: T,
    values: Iterable<Pair<T, String>>,
    label: String,
    onItemSelected: (value: T) -> Unit,
) {
    var expanded by remember { mutableStateOf(false) }

    ExposedDropdownMenuBox(
        expanded = expanded,
        onExpandedChange = { expanded = !expanded },
    ) {
        TextField(
            modifier = Modifier.menuAnchor(),
            readOnly = true,
            value = value.toString(),
            onValueChange = {},
            label = { Text(label) },
            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
            colors = ExposedDropdownMenuDefaults.textFieldColors(),
        )
        ExposedDropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false },
        ) {
            values.forEach { valueAndText ->
                val itemValue = valueAndText.first
                val itemText = valueAndText.second
                DropdownMenuItem(
                    text = { Text(itemText) },
                    onClick = {
                        expanded = false
                        onItemSelected(itemValue)
                    },
                )
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        var value by remember { mutableStateOf(1) }
        Dropdown(value = value,
            values = listOf(Pair(1, "Test"), Pair(2, "Test 2"), Pair(3, "Test 3")),
            label = "Test Dropdown",
            onItemSelected = {
                value = it
            })
    }
}
