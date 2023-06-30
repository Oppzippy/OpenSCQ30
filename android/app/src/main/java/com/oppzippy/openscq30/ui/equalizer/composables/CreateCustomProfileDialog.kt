package com.oppzippy.openscq30.ui.equalizer.composables

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfile

@OptIn(ExperimentalMaterial3Api::class)
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
                    if (existingProfiles.none {it.name == profileName}) {
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
