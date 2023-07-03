package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfile

@Composable
fun CreateCustomProfileDialog(
    isOpen: Boolean,
    onDismiss: () -> Unit,
    onCreateCustomProfile: (name: String) -> Unit,
    existingProfiles: List<CustomProfile>,
) {
    if (isOpen) {
        var profileName by remember { mutableStateOf("") }
        AlertDialog(
            onDismissRequest = onDismiss,
            title = {
                Text(text = stringResource(R.string.new_custom_profile))
            },
            text = {
                TextField(
                    value = profileName,
                    label = { Text(stringResource(R.string.name)) },
                    onValueChange = { profileName = it },
                    modifier = Modifier.fillMaxWidth(),
                )
            },
            confirmButton = {
                Button(onClick = {
                    onCreateCustomProfile(profileName)
                    onDismiss()
                }) {
                    if (existingProfiles.none { it.name == profileName }) {
                        Text(stringResource(R.string.create))
                    } else {
                        Text(stringResource(R.string.replace))
                    }
                }
            },
            dismissButton = {
                Button(onClick = onDismiss) {
                    Text(stringResource(R.string.cancel))
                }
            },
        )
    }
}
