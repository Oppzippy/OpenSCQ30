package com.oppzippy.openscq30.ui.utils

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * @param icon Should be 24dp
 * @param text Primary text
 * @param otherText Text to display off to the right side
 */
@Composable
fun NavItem(modifier: Modifier = Modifier, icon: @Composable () -> Unit, text: String, otherText: String? = null) {
    Row(
        modifier = modifier
            .padding(16.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        icon()
        Text(modifier = Modifier.weight(1f), text = text)
        if (otherText != null) {
            Text(otherText)
        }
    }
}
