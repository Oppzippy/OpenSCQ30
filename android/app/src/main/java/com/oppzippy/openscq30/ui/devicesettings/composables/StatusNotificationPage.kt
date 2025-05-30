package com.oppzippy.openscq30.ui.devicesettings.composables

import android.Manifest
import android.content.Intent
import android.os.Build
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification
import com.oppzippy.openscq30.ui.utils.PermissionCheck

@Composable
fun StatusNotificationPage(
    quickPresets: List<String>,
    quickPresetSlots: List<String?>,
    onQuickPresetSlotChange: (Int, String?) -> Unit,
) {
    val context = LocalContext.current
    val inner = @Composable { StatusNotificationPageContent(quickPresets, quickPresetSlots, onQuickPresetSlotChange) }

    // Older android versions don't require any special permission for foreground service notifications
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
        PermissionCheck(
            prompt = stringResource(R.string.notification_permission_is_required),
            permission = Manifest.permission.POST_NOTIFICATIONS,
            onPermissionGranted = {
                // Since we may have not had notification permission before this point, we need to resend the
                // notification to ensure it is visible.
                context.sendBroadcast(Intent().apply { action = SoundcoreDeviceNotification.ACTION_SEND_NOTIFICATION })
            },
        ) {
            inner()
        }
    } else {
        inner()
    }
}

@Composable
private fun StatusNotificationPageContent(
    quickPresets: List<String>,
    quickPresetSlots: List<String?>,
    onQuickPresetSlotChange: (Int, String?) -> Unit,
) {
    Column {
        quickPresetSlots.forEachIndexed { index, slot ->
            com.oppzippy.openscq30.ui.utils.OptionalSelect(
                name = stringResource(R.string.quick_preset_slot_x, index + 1),
                options = quickPresets,
                onSelect = { onQuickPresetSlotChange(index, it?.let { quickPresets[it] }) },
                selectedIndex = quickPresets.indexOf(slot),
            )
        }
    }
}
