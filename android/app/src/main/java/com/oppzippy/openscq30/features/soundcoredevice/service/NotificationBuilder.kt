package com.oppzippy.openscq30.features.soundcoredevice.service

import android.app.Notification
import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.graphics.Bitmap
import android.graphics.drawable.Icon
import com.oppzippy.openscq30.MainActivity
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.visualization.EqualizerLine
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_DISCONNECT
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.ACTION_QUICK_PRESET
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.INTENT_PRESET_NUMBER
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification.NOTIFICATION_CHANNEL_ID
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.libextensions.resources.toStringResource
import dagger.hilt.android.scopes.ServiceScoped
import javax.inject.Inject

@ServiceScoped
class NotificationBuilder @Inject constructor(private val context: Service) {
    operator fun invoke(
        status: ConnectionStatus,
        quickPresetNames: List<String?>,
    ): Notification {
        val openAppIntent = Intent(context, MainActivity::class.java)
        openAppIntent.addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)

        val builder = Notification.Builder(context, NOTIFICATION_CHANNEL_ID).setOngoing(true)
            .setOnlyAlertOnce(true).setSmallIcon(R.drawable.headphones).setLargeIcon(
                if (status is ConnectionStatus.Connected) {
                    val bitmap = Bitmap.createBitmap(100, 100, Bitmap.Config.ARGB_8888)
                    val volumeAdjustments =
                        status.device.state.equalizerConfiguration().volumeAdjustments()
                            .adjustments()
                    EqualizerLine(volumeAdjustments.toList()).drawBitmap(
                        bitmap = bitmap,
                        yOffset = bitmap.height / 4F,
                        height = bitmap.height / 2F,
                    )
                    bitmap
                } else {
                    null
                },
            ).setContentTitle(
                when (status) {
                    is ConnectionStatus.AwaitingConnection -> context.getString(R.string.awaiting_connection)
                    is ConnectionStatus.Connected -> context.getString(R.string.connected_to)
                        .format(
                            status.device.name,
                        )

                    is ConnectionStatus.Connecting -> context.getString(R.string.connecting_to)
                        .format(
                            status.macAddress,
                        )

                    ConnectionStatus.Disconnected -> context.getString(R.string.disconnected)
                },
            ).setContentText(
                if (status is ConnectionStatus.Connected) {
                    val deviceState = status.device.state
                    if (deviceState.ambientSoundMode() == AmbientSoundMode.NoiseCanceling) {
                        context.getString(
                            R.string.ambient_sound_mode_and_noise_canceling_mode_values,
                            context.getString(
                                deviceState.ambientSoundMode().toStringResource(),
                            ),
                            context.getString(
                                deviceState.noiseCancelingMode().toStringResource(),
                            ),
                        )
                    } else {
                        context.getString(deviceState.ambientSoundMode().toStringResource())
                    }
                } else {
                    null
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
                            action = ACTION_DISCONNECT
                        },
                        PendingIntent.FLAG_IMMUTABLE,
                    ),
                ).build(),
            ).addAction(
                buildQuickPresetNotificationAction(
                    presetNumber = 1,
                    name = quickPresetNames.getOrNull(0),
                    icon = Icon.createWithResource(context, R.drawable.counter_1_48px),
                ),
            ).addAction(
                buildQuickPresetNotificationAction(
                    presetNumber = 2,
                    name = quickPresetNames.getOrNull(1),
                    icon = Icon.createWithResource(context, R.drawable.counter_2_48px),
                ),
            )
        return builder.build()
    }

    private fun buildQuickPresetNotificationAction(
        presetNumber: Int,
        name: String?,
        icon: Icon,
    ): Notification.Action {
        return Notification.Action.Builder(
            icon,
            name ?: context.getString(R.string.quick_preset_number, presetNumber),
            PendingIntent.getBroadcast(
                context,
                1 + presetNumber,
                Intent().apply {
                    action = ACTION_QUICK_PRESET
                    putExtra(INTENT_PRESET_NUMBER, presetNumber)
                },
                PendingIntent.FLAG_IMMUTABLE,
            ),
        ).build()
    }
}
