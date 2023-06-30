package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.AlertDialogDefaults
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
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfile

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ReplaceCustomProfileDialog(
    isOpen: Boolean,
    profiles: List<CustomProfile>,
    onProfileSelected: (profile: CustomProfile) -> Unit,
    onDismiss: () -> Unit,
) {
    if (isOpen) {
        AlertDialog(onDismissRequest = onDismiss) {
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
                            start = 16.dp, end = 16.dp, bottom = 16.dp,
                        )
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
