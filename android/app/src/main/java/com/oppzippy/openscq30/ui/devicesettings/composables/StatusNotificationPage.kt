package com.oppzippy.openscq30.ui.devicesettings.composables

import android.Manifest
import android.content.Intent
import android.os.Build
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.ui.utils.PermissionCheck

@Composable
fun StatusNotificationPage(
    settingIds: List<String>,
    featuredSettingSlots: List<String?>,
    onFeaturedSettingSlotChange: (Int, String?) -> Unit,
    quickPresets: List<String>,
    quickPresetSlots: List<String?>,
    onQuickPresetSlotChange: (Int, String?) -> Unit,
) {
    val context = LocalContext.current
    val inner = @Composable {
        StatusNotificationPageContent(
            settingIds = settingIds,
            featuredSettingSlots = featuredSettingSlots,
            onFeaturedSettingSlotChange = onFeaturedSettingSlotChange,
            quickPresets = quickPresets,
            quickPresetSlots = quickPresetSlots,
            onQuickPresetSlotChange = onQuickPresetSlotChange,
        )
    }

    // Older android versions don't require any special permission for foreground service notifications
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
        PermissionCheck(
            prompt = stringResource(R.string.notification_permission_is_required),
            permission = Manifest.permission.POST_NOTIFICATIONS,
            onPermissionGranted = {
                // Since we may have not had notification permission before this point, we need to resend the
                // notification to ensure it is visible.
                context.sendBroadcast(
                    Intent().apply {
                        action = SoundcoreDeviceNotification.ACTION_SEND_NOTIFICATION
                        `package` = context.packageName
                    },
                )
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
    settingIds: List<String>,
    featuredSettingSlots: List<String?>,
    onFeaturedSettingSlotChange: (Int, String?) -> Unit,
    quickPresets: List<String>,
    quickPresetSlots: List<String?>,
    onQuickPresetSlotChange: (Int, String?) -> Unit,
) {
    Column {
        featuredSettingSlots.forEachIndexed { index, settingId ->
            com.oppzippy.openscq30.ui.utils.OptionalSelect(
                name = stringResource(R.string.featured_setting_x, index + 1),
                options = settingIds.map { translateSettingId(it) },
                onSelect = { onFeaturedSettingSlotChange(index, it?.let { settingIds[it] }) },
                selectedIndex = settingIds.indexOf(settingId),
            )
        }

        Spacer(Modifier.height(5.dp))

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
