package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.AlertDialogDefaults
import androidx.compose.material3.BasicAlertDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ProvideTextStyle
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ReplaceCustomProfileDialog(
    isOpen: Boolean,
    profiles: List<CustomProfile>,
    onProfileSelected: (profile: CustomProfile) -> Unit,
    onDismiss: () -> Unit,
) {
    if (isOpen) {
        BasicAlertDialog(onDismissRequest = onDismiss) {
            Surface(
                modifier = Modifier
                    .wrapContentWidth()
                    .wrapContentHeight(),
                shape = MaterialTheme.shapes.large,
                tonalElevation = AlertDialogDefaults.TonalElevation,
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    ProvideTextStyle(MaterialTheme.typography.titleLarge) {
                        Text(
                            text = stringResource(R.string.replace_existing_profile),
                            modifier = Modifier.padding(16.dp),
                        )
                    }
                    LazyColumn(
                        modifier = Modifier.padding(
                            start = 16.dp,
                            end = 16.dp,
                            bottom = 16.dp,
                        ),
                    ) {
                        items(profiles) { profile ->
                            TextButton(
                                modifier = Modifier.fillMaxWidth(),
                                onClick = {
                                    onDismiss()
                                    onProfileSelected(profile)
                                },
                            ) {
                                Text(profile.name)
                            }
                        }
                    }
                }
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewReplaceCustomProfileDialog() {
    OpenSCQ30Theme {
        ReplaceCustomProfileDialog(
            isOpen = true,
            profiles = listOf(
                CustomProfile("Test Profile 1", 0.0, -1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0),
                CustomProfile("Test Profile 2", -5.0, -2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0),
            ),
            onDismiss = {},
            onProfileSelected = {},
        )
    }
}
