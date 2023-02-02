package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import kotlin.math.pow
import kotlin.math.roundToInt

@Composable
fun Equalizer(
    values: List<Byte>, onValueChange: (index: Int, value: Byte) -> Unit, enabled: Boolean = true,
) {
    if (values.size != 8) {
        throw IllegalArgumentException("There must be exactly 8 values")
    }
    LazyColumn(
        userScrollEnabled = true,
    ) {
        values.forEachIndexed { index, value ->
            item {
                EqualizerSlider(
                    hz = (100 * 2F.pow(index)).roundToInt(),
                    value = value,
                    onValueChange = { value ->
                        onValueChange(index, value)
                    },
                    enabled = enabled,
                )
                Divider(modifier = Modifier.padding(vertical = 4.dp))
            }
        }
    }
}


@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    var values by remember { mutableStateOf(listOf<Byte>(0, 0, 0, 0, 0, 0, 0, 0)) }
    OpenSCQ30Theme {
        Equalizer(
            values = values,
            onValueChange = { changedIndex, changedValue ->
                values = values.mapIndexed { index, value ->
                    return@mapIndexed if (index == changedIndex) {
                        changedValue
                    } else {
                        value
                    }
                }
            },
        )
    }
}
