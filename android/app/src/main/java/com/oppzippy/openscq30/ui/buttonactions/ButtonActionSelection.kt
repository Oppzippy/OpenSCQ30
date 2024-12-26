package com.oppzippy.openscq30.ui.buttonactions

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.wrapper.ButtonAction
import com.oppzippy.openscq30.lib.wrapper.ButtonConfiguration
import com.oppzippy.openscq30.lib.wrapper.MultiButtonConfiguration
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun ButtonActionSelection(buttonConfiguration: MultiButtonConfiguration, onChange: (MultiButtonConfiguration) -> Unit) {
    val scrollState = rememberScrollState()
    Column(
        Modifier
            .widthIn(max = 600.dp)
            .fillMaxHeight()
            .verticalScroll(scrollState),
    ) {
        ButtonActionRow(
            stringResource(R.string.left_single_click),
            buttonConfiguration.leftSingleClick,
            onChange = { onChange(buttonConfiguration.copy(leftSingleClick = it)) },
        )
        ButtonActionRow(
            stringResource(R.string.left_double_click),
            buttonConfiguration.leftDoubleClick,
            onChange = { onChange(buttonConfiguration.copy(leftDoubleClick = it)) },
        )
        ButtonActionRow(
            stringResource(R.string.left_long_press),
            buttonConfiguration.leftLongPress,
            onChange = { onChange(buttonConfiguration.copy(leftLongPress = it)) },
        )
        ButtonActionRow(
            stringResource(R.string.right_single_click),
            buttonConfiguration.rightSingleClick,
            onChange = { onChange(buttonConfiguration.copy(rightSingleClick = it)) },
        )
        ButtonActionRow(
            stringResource(R.string.right_double_click),
            buttonConfiguration.rightDoubleClick,
            onChange = { onChange(buttonConfiguration.copy(rightDoubleClick = it)) },
        )
        ButtonActionRow(
            stringResource(R.string.right_long_press),
            buttonConfiguration.rightLongPress,
            onChange = { onChange(buttonConfiguration.copy(rightLongPress = it)) },
        )
    }
}

@Composable
private fun ButtonActionRow(label: String, state: ButtonConfiguration, onChange: (ButtonConfiguration) -> Unit) {
    var isDialogOpen by remember { mutableStateOf(false) }
    if (isDialogOpen) {
        ButtonActionSelectionDialog(
            onDismissRequest = { isDialogOpen = false },
            onActionSelected = { newAction ->
                isDialogOpen = false
                onChange(
                    if (newAction != null) {
                        ButtonConfiguration(true, newAction)
                    } else {
                        ButtonConfiguration(false, state.action)
                    },
                )
            },
        )
    }

    Box(contentAlignment = Alignment.Center) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.Center,
        ) {
            Box(Modifier.weight(1f), Alignment.CenterEnd) {
                Text(label)
            }
            Spacer(Modifier.width(10.dp))
            Button(onClick = { isDialogOpen = true }, Modifier.weight(1f)) {
                if (state.isEnabled) {
                    Text(stringResource(state.action.toStringResource()))
                } else {
                    Text(stringResource(R.string.disabled))
                }
            }
        }
    }
}

@Composable
private fun ButtonActionSelectionDialog(onDismissRequest: () -> Unit, onActionSelected: (ButtonAction?) -> Unit) {
    Dialog(onDismissRequest = onDismissRequest) {
        Card(
            Modifier
                .fillMaxWidth()
                .padding(16.dp),
            shape = RoundedCornerShape(16.dp),
        ) {
            LazyColumn {
                item {
                    TextButton(onClick = { onActionSelected(null) }, Modifier.fillMaxWidth()) {
                        Text(stringResource(R.string.disabled))
                    }
                }
                items(ButtonAction.entries) {
                    TextButton({ onActionSelected(it) }, Modifier.fillMaxWidth()) {
                        Text(stringResource(it.toStringResource()))
                    }
                }
            }
        }
    }
}

@Preview
@Composable
fun PreviewCustomButtons() {
    OpenSCQ30Theme {
        ButtonActionSelection(
            buttonConfiguration = MultiButtonConfiguration(
                ButtonConfiguration(false, ButtonAction.GameMode),
                ButtonConfiguration(true, ButtonAction.NextSong),
                ButtonConfiguration(true, ButtonAction.VolumeDown),
                ButtonConfiguration(true, ButtonAction.PreviousSong),
                ButtonConfiguration(true, ButtonAction.VoiceAssistant),
                ButtonConfiguration(true, ButtonAction.AmbientSoundMode),
            ),
            onChange = {},
        )
    }
}
