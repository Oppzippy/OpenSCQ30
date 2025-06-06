package com.oppzippy.openscq30.ui.devicesettings.composables

import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfile

@Composable
fun MigrateLegacyEqualizerProfilesPage(
    legacyEqualizerProfiles: List<LegacyEqualizerProfile>,
    onMigrateLegacyEqualizerProfile: (LegacyEqualizerProfile) -> Unit,
) {
    LazyColumn {
        items(legacyEqualizerProfiles) { profile ->
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(modifier = Modifier.weight(1f), text = profile.name)
                Button(onClick = {
                    onMigrateLegacyEqualizerProfile(profile)
                }) { Text(stringResource(R.string.migrate)) }
            }
        }
    }
}
