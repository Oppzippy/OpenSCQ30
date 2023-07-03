package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.foundation.layout.width
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExposedDropdownMenuBox
import androidx.compose.material3.ExposedDropdownMenuDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

data class DropdownOption<T>(
    val value: T,
    val label: @Composable () -> Unit,
    val name: String,
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun <T> Dropdown(
    value: T?,
    options: Iterable<DropdownOption<T>>,
    label: String,
    onItemSelected: (value: T) -> Unit,
) {
    var expanded by remember { mutableStateOf(false) }

    ExposedDropdownMenuBox(expanded = expanded, onExpandedChange = { expanded = !expanded }) {
        TextField(
            modifier = Modifier
                .menuAnchor()
                .width(TextFieldDefaults.MinWidth),
            readOnly = true,
            value = options.find { it.value == value }?.name ?: stringResource(R.string.none),
            onValueChange = {},
            label = { Text(label) },
            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
            colors = ExposedDropdownMenuDefaults.textFieldColors(),
        )
        ExposedDropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            options.forEach { option ->
                DropdownMenuItem(text = { option.label() }, onClick = {
                    expanded = false
                    onItemSelected(option.value)
                })
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        var value by remember { mutableStateOf(1) }
        Dropdown(
            value = value,
            options = listOf(
                DropdownOption(value = 1, label = { Text("Test") }, name = "1"),
                DropdownOption(value = 2, label = { Text("Test 2") }, name = "2"),
                DropdownOption(value = 3, label = { Text("Test 3") }, name = "3"),
            ),
            label = "Test Dropdown",
            onItemSelected = {
                value = it
            },
        )
    }
}
