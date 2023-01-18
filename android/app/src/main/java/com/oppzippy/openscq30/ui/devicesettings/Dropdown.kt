package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun <T> Dropdown(
    value: T,
    values: Iterable<T>,
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
            values.forEach { newValue ->
                DropdownMenuItem(
                    text = { Text(newValue.toString()) },
                    onClick = {
                        expanded = false
                        onItemSelected(newValue)
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
        var value by remember { mutableStateOf("Test") }
        Dropdown(value = value,
            values = listOf("Test", "Test 2", "Test 3"),
            label = "Test Dropdown",
            onItemSelected = {
                value = it
            })
    }
}
