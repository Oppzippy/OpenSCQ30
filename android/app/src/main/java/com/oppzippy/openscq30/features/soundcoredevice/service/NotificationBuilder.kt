package com.oppzippy.openscq30.features.soundcoredevice.service

import android.app.Notification
import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.graphics.drawable.Icon
import com.oppzippy.openscq30.MainActivity
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.lib.bindings.translateValue
import dagger.hilt.android.scopes.ServiceScoped
import javax.inject.Inject

@ServiceScoped
class NotificationBuilder @Inject constructor(private val context: Service) {
    operator fun invoke(
        status: ConnectionStatus,
        quickPresetNames: List<String?>,
        featuredSettingIds: List<String?>,
    ): Notification {
        val openAppIntent = Intent(context, MainActivity::class.java)
        openAppIntent.addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)

        val device = if (status is ConnectionStatus.Connected) {
            status.deviceManager.device
        } else {
            null
        }

        val builder = Notification.Builder(context, DeviceService.NOTIFICATION_CHANNEL_ID).setOngoing(true)
            .setOnlyAlertOnce(true).setSmallIcon(R.drawable.headphones).setContentTitle(
                when (status) {
                    is ConnectionStatus.AwaitingConnection -> context.getString(R.string.awaiting_connection)

                    is ConnectionStatus.Connected -> context.getString(
                        R.string.connected_to,
                        translateDeviceModel(status.deviceManager.device.model()),
                    )

                    is ConnectionStatus.Connecting -> context.getString(R.string.connecting_to, status.macAddress)

                    ConnectionStatus.Disconnected -> context.getString(R.string.disconnected)
                },
            ).setContentText(
                device?.let {
                    featuredSettingIds.filterNotNull()
                        .mapNotNull { device.setting(it)?.let { setting -> Pair(it, setting) } }
                        .joinToString(separator = "\n") { (settingId, setting) ->
                            context.getString(
                                R.string.setting_id_with_value,
                                translateSettingId(settingId),
                                translateValue(setting, setting.toValue()),
                            )
                        }
                },
            ).setContentIntent(
                PendingIntent.getActivity(
                    context,
                    1,
                    openAppIntent,
                    PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT,
                ),
            ).addAction(
                Notification.Action.Builder(
                    Icon.createWithResource(context, R.drawable.baseline_headset_off_24),
                    context.getString(R.string.disconnect),
                    PendingIntent.getBroadcast(
                        context,
                        1,
                        Intent().apply {
                            action = DeviceService.ACTION_DISCONNECT
                            `package` = context.packageName
                        },
                        PendingIntent.FLAG_IMMUTABLE,
                    ),
                ).build(),
            )

        quickPresetNames.take(2).filterNotNull().forEachIndexed { index, name ->
            builder.addAction(
                buildQuickPresetNotificationAction(
                    index,
                    name,
                    icon = Icon.createWithResource(
                        context,
                        if (index == 0) R.drawable.counter_1_48px else R.drawable.counter_2_48px,
                    ),
                ),
            )
        }

        return builder.build()
    }

    private fun buildQuickPresetNotificationAction(presetId: Int, name: String, icon: Icon): Notification.Action =
        Notification.Action.Builder(
            icon,
            name,
            PendingIntent.getBroadcast(
                context,
                presetId + 2,
                Intent().apply {
                    action = DeviceService.ACTION_QUICK_PRESET
                    `package` = context.packageName
                    putExtra(DeviceService.INTENT_EXTRA_PRESET_ID, presetId)
                },
                PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT,
            ),
        ).build()
}
