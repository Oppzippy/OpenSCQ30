package com.oppzippy.openscq30.features.soundcoredevice.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Paint
import android.graphics.drawable.Icon
import android.os.Binder
import android.os.IBinder
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.lifecycleScope
import com.oppzippy.openscq30.MainActivity
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetIdAndName
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.VolumeAdjustments
import com.oppzippy.openscq30.libextensions.resources.toStringResource
import com.oppzippy.openscq30.ui.equalizer.models.EqualizerLine
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfileDao
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import javax.inject.Inject
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Duration.Companion.seconds

@AndroidEntryPoint
class DeviceService : LifecycleService() {
    companion object {
        private const val NOTIFICATION_CHANNEL_ID =
            "com.oppzippy.openscq30.notification.DeviceServiceChannel"
        private const val NOTIFICATION_ID = 1
        private const val ACTION_QUICK_PRESET = "com.oppzippy.openscq30.broadcast.QuickPreset"
        private const val ACTION_DISCONNECT = "com.oppzippy.openscq30.broadcast.Disconnect"
        private const val INTENT_PRESET_NUMBER = "com.oppzippy.openscq30.presetNumber"

        /** Intent extra for setting mac address when launching service */
        const val MAC_ADDRESS = "com.oppzippy.openscq30.macAddress"
    }

    @Inject
    lateinit var factory: SoundcoreDeviceFactory
    lateinit var connectionManager: DeviceConnectionManager

    @Inject
    lateinit var quickPresetDao: QuickPresetDao

    @Inject
    lateinit var customProfileDao: CustomProfileDao

    private val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            when (intent?.action) {
                ACTION_DISCONNECT -> {
                    MainScope().launch {
                        // Disconnecting will trigger stopping this service
                        connectionManager.disconnect()
                    }
                }

                ACTION_QUICK_PRESET -> {
                    val presetNumber = intent.getIntExtra(INTENT_PRESET_NUMBER, 0)
                    lifecycleScope.launch {
                        quickPresetDao.get(presetNumber)?.let { quickPreset ->
                            val ambientSoundMode = quickPreset.ambientSoundMode
                            val noiseCancelingMode = quickPreset.noiseCancelingMode
                            val equalizerConfiguration = quickPreset.equalizerProfileName?.let {
                                customProfileDao.get(it)
                            }?.let {
                                EqualizerConfiguration(VolumeAdjustments(it.values.toByteArray()))
                            }

                            // Set them both in one go if possible to maybe save a packet
                            if (ambientSoundMode != null && noiseCancelingMode != null) {
                                connectionManager.setSoundMode(ambientSoundMode, noiseCancelingMode)
                            } else {
                                ambientSoundMode?.let { connectionManager.setAmbientSoundMode(it) }
                                noiseCancelingMode?.let { connectionManager.setNoiseCancelingMode(it) }
                            }
                            equalizerConfiguration?.let {
                                connectionManager.setEqualizerConfiguration(it)
                            }
                        }
                    }
                }
            }
        }
    }

    private lateinit var quickPresetNames: StateFlow<List<QuickPresetIdAndName>>

    override fun onCreate() {
        super.onCreate()
        connectionManager = DeviceConnectionManager(factory, lifecycleScope)

        quickPresetNames =
            quickPresetDao.allNames().stateIn(lifecycleScope, SharingStarted.Eagerly, emptyList())

        lifecycleScope.launch {
            connectionManager.connectionStatusFlow.first { it is ConnectionStatus.Disconnected }
            stopSelf()
        }

        val filter = IntentFilter(ACTION_DISCONNECT).apply {
            addAction(ACTION_QUICK_PRESET)
        }
        registerReceiver(broadcastReceiver, filter)
    }

    override fun onDestroy() {
        super.onDestroy()
        unregisterReceiver(broadcastReceiver)

        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.cancel(NOTIFICATION_ID)

        MainScope().launch {
            connectionManager.disconnect()
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)

        createNotificationChannel()
        val notification = buildNotification()
        startForeground(NOTIFICATION_ID, notification)

        lifecycleScope.launch {
            connectionManager.connectionStatusFlow.collectLatest {
                if (it is ConnectionStatus.Connected) {
                    it.device.stateFlow.debounce(500.milliseconds).collectLatest {
                        updateNotification()
                    }
                }
            }
        }
        lifecycleScope.launch {
            quickPresetNames.debounce(1.seconds).collectLatest { updateNotification() }
        }

        intent?.getStringExtra(MAC_ADDRESS)?.let { macAddress ->
            lifecycleScope.launch {
                setMacAddress(macAddress)
            }
        }

        return START_REDELIVER_INTENT
    }

    private fun createNotificationChannel() {
        val channel = NotificationChannel(
            NOTIFICATION_CHANNEL_ID,
            "Device Connection Service",
            NotificationManager.IMPORTANCE_LOW,
        )
        channel.enableVibration(false)
        channel.enableLights(false)
        channel.setSound(null, null)
        val manager = getSystemService(NotificationManager::class.java)
        manager.createNotificationChannel(channel)
    }

    private fun updateNotification() {
        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.notify(NOTIFICATION_ID, buildNotification())
    }

    private fun buildNotification(): Notification {
        val openAppIntent = Intent(this, MainActivity::class.java)
        openAppIntent.addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)

        val status = connectionManager.connectionStatusFlow.value

        val quickPresets = quickPresetNames.value.let { quickPresets ->
            Pair(
                quickPresets.firstOrNull { it.id == 0 },
                quickPresets.firstOrNull { it.id == 1 },
            )
        }

        val builder = Notification.Builder(this, NOTIFICATION_CHANNEL_ID).setOngoing(true)
            .setOnlyAlertOnce(true).setSmallIcon(R.drawable.headphones).setLargeIcon(
                if (status is ConnectionStatus.Connected) {
                    buildEqualizerBitmap(status)
                } else {
                    null
                },
            ).setContentTitle(
                when (status) {
                    is ConnectionStatus.AwaitingConnection -> getString(R.string.awaiting_connection)
                    is ConnectionStatus.Connected -> getString(R.string.connected_to).format(
                        status.device.name,
                    )

                    is ConnectionStatus.Connecting -> getString(R.string.connecting_to).format(
                        status.macAddress,
                    )

                    ConnectionStatus.Disconnected -> getString(R.string.disconnected)
                },
            ).setContentText(
                if (status is ConnectionStatus.Connected) {
                    val deviceState = status.device.state
                    if (deviceState.ambientSoundMode() == AmbientSoundMode.NoiseCanceling) {
                        getString(
                            R.string.ambient_sound_mode_and_noise_canceling_mode_values,
                            getString(deviceState.ambientSoundMode().toStringResource()),
                            getString(deviceState.noiseCancelingMode().toStringResource()),
                        )
                    } else {
                        getString(deviceState.ambientSoundMode().toStringResource())
                    }
                } else {
                    null
                },
            ).setContentIntent(
                PendingIntent.getActivity(
                    this,
                    1,
                    openAppIntent,
                    PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT,
                ),
            ).addAction(
                Notification.Action.Builder(
                    Icon.createWithResource(this, R.drawable.baseline_headset_off_24),
                    getString(R.string.disconnect),
                    PendingIntent.getBroadcast(
                        this,
                        1,
                        Intent().apply {
                            action = ACTION_DISCONNECT
                        },
                        PendingIntent.FLAG_IMMUTABLE,
                    ),
                ).build(),
            ).addAction(
                Notification.Action.Builder(
                    Icon.createWithResource(this, R.drawable.counter_1_48px),
                    quickPresets.first?.name ?: getString(R.string.quick_preset_number, 1),
                    PendingIntent.getBroadcast(
                        this,
                        2,
                        Intent().apply {
                            action = ACTION_QUICK_PRESET
                            putExtra(INTENT_PRESET_NUMBER, 0)
                        },
                        PendingIntent.FLAG_IMMUTABLE,
                    ),
                ).build(),
            ).addAction(
                Notification.Action.Builder(
                    Icon.createWithResource(this, R.drawable.counter_2_48px),
                    quickPresets.second?.name ?: getString(R.string.quick_preset_number, 2),
                    PendingIntent.getBroadcast(
                        this,
                        3,
                        Intent().apply {
                            action = ACTION_QUICK_PRESET
                            putExtra(INTENT_PRESET_NUMBER, 1)
                        },
                        PendingIntent.FLAG_IMMUTABLE,
                    ),
                ).build(),
            )
        return builder.build()
    }

    private fun buildEqualizerBitmap(status: ConnectionStatus.Connected): Bitmap {
        val bitmap = Bitmap.createBitmap(100, 100, Bitmap.Config.ARGB_8888)
        val equalizerConfiguration = status.device.state.equalizerConfiguration()
        val canvas = Canvas(bitmap)
        val line = EqualizerLine(equalizerConfiguration.volumeAdjustments().adjustments().toList())
        val points = line.draw(canvas.width.toFloat(), canvas.height.toFloat() / 2F, 4F)
        val lineCoordinates = points.flatMapIndexed { index, pair ->
            val scaledY = pair.second + canvas.height / 4
            if (index == 0 || index == points.size - 1) {
                listOf(pair.first, scaledY)
            } else {
                listOf(pair.first, scaledY, pair.first, scaledY)
            }
        }
        canvas.drawLines(
            lineCoordinates.toFloatArray(),
            Paint(Paint.ANTI_ALIAS_FLAG).apply {
                strokeWidth = bitmap.height * 0.05F
                color = 0xFF777777.toInt()
                strokeCap = Paint.Cap.ROUND
                strokeJoin = Paint.Join.ROUND
            },
        )
        return bitmap
    }

    fun doesNotificationExist(): Boolean {
        val notificationManager =
            applicationContext.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        val doesNotificationExist = notificationManager.activeNotifications.any {
            (it.notification.channelId == NOTIFICATION_CHANNEL_ID) && (it.id == NOTIFICATION_ID)
        }
        return doesNotificationExist
    }

    private val binder = MyBinder()

    override fun onBind(intent: Intent): IBinder {
        super.onBind(intent)
        return binder
    }

    private suspend fun setMacAddress(macAddress: String?) {
        if (macAddress != null) {
            connectionManager.connect(macAddress)
        } else {
            connectionManager.disconnect()
        }
    }

    inner class MyBinder : Binder() {
        fun getService(): DeviceService = this@DeviceService
    }
}
